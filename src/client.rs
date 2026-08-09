//! HTTP client configuration and transport behavior.

use std::{
    fmt,
    time::{Duration, SystemTime},
};

use reqwest::{
    Method, StatusCode, Url,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue, RETRY_AFTER},
};
use secrecy::{ExposeSecret, SecretString};
use serde::{Serialize, de::DeserializeOwned};

use crate::error::{ApiError, Error, ErrorEnvelope};

const DEFAULT_BASE_URL: &str = "https://api.heyrafiki.space/v1/";

/// Retry settings for retryable API responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    max_retries: u8,
    base_delay: Duration,
    max_delay: Duration,
}

impl RetryPolicy {
    /// Creates a retry policy.
    pub fn new(max_retries: u8, base_delay: Duration, max_delay: Duration) -> Result<Self, Error> {
        if base_delay.is_zero() {
            return Err(Error::InvalidInput {
                field: "retry_policy.base_delay",
                reason: "must be greater than zero",
            });
        }
        if max_delay < base_delay {
            return Err(Error::InvalidInput {
                field: "retry_policy.max_delay",
                reason: "must be at least base_delay",
            });
        }
        Ok(Self {
            max_retries,
            base_delay,
            max_delay,
        })
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 2,
            base_delay: Duration::from_millis(250),
            max_delay: Duration::from_secs(30),
        }
    }
}

/// Pagination options for list operations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ListParams {
    limit: Option<u8>,
}

impl ListParams {
    /// Requests between 1 and 100 records.
    pub fn new(limit: u8) -> Result<Self, Error> {
        if !(1..=100).contains(&limit) {
            return Err(Error::InvalidInput {
                field: "limit",
                reason: "must be between 1 and 100",
            });
        }
        Ok(Self { limit: Some(limit) })
    }

    pub(crate) fn apply(self, url: &mut Url) {
        if let Some(limit) = self.limit {
            url.query_pairs_mut()
                .append_pair("limit", &limit.to_string());
        }
    }
}

/// Options required by an idempotent write operation.
#[derive(Clone)]
pub struct WriteOptions {
    pub(crate) idempotency_key: HeaderValue,
}

impl WriteOptions {
    /// Validates a caller-owned idempotency key against the public contract.
    pub fn new(key: impl AsRef<str>) -> Result<Self, Error> {
        let key = key.as_ref();
        if !(8..=255).contains(&key.len()) {
            return Err(Error::InvalidInput {
                field: "idempotency_key",
                reason: "must contain between 8 and 255 bytes",
            });
        }
        let mut value = HeaderValue::from_str(key).map_err(|_| Error::InvalidInput {
            field: "idempotency_key",
            reason: "must be a valid HTTP header value",
        })?;
        value.set_sensitive(true);
        Ok(Self {
            idempotency_key: value,
        })
    }
}

impl fmt::Debug for WriteOptions {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WriteOptions")
            .finish_non_exhaustive()
    }
}

/// Options required when recording a coverage batch.
#[derive(Clone)]
pub struct CoverageBatchOptions {
    pub(crate) write: WriteOptions,
    pub(crate) artifact_reference: HeaderValue,
}

impl CoverageBatchOptions {
    /// Creates options with an idempotency key and stable source-artifact reference.
    pub fn new(
        idempotency_key: impl AsRef<str>,
        artifact_reference: impl AsRef<str>,
    ) -> Result<Self, Error> {
        let artifact_reference = artifact_reference.as_ref();
        if artifact_reference.is_empty() || artifact_reference.len() > 190 {
            return Err(Error::InvalidInput {
                field: "artifact_reference",
                reason: "must contain between 1 and 190 bytes",
            });
        }
        let mut characters = artifact_reference.chars();
        let valid_first = characters
            .next()
            .is_some_and(|value| value.is_ascii_alphanumeric());
        let valid_rest = characters.all(|value| {
            value.is_ascii_alphanumeric() || matches!(value, ':' | '.' | '_' | '/' | '-')
        });
        if !valid_first || !valid_rest {
            return Err(Error::InvalidInput {
                field: "artifact_reference",
                reason: "must match the public artifact-reference format",
            });
        }
        let mut value =
            HeaderValue::from_str(artifact_reference).map_err(|_| Error::InvalidInput {
                field: "artifact_reference",
                reason: "must be a valid HTTP header value",
            })?;
        value.set_sensitive(true);
        Ok(Self {
            write: WriteOptions::new(idempotency_key)?,
            artifact_reference: value,
        })
    }
}

impl fmt::Debug for CoverageBatchOptions {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CoverageBatchOptions")
            .finish_non_exhaustive()
    }
}

/// Builder for a [`Client`].
pub struct ClientBuilder {
    api_key: SecretString,
    base_url: String,
    retry_policy: RetryPolicy,
    http_client: Option<reqwest::Client>,
}

impl ClientBuilder {
    /// Uses a custom API base URL.
    #[must_use]
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Uses a custom retry policy.
    #[must_use]
    pub fn retry_policy(mut self, retry_policy: RetryPolicy) -> Self {
        self.retry_policy = retry_policy;
        self
    }

    /// Uses a caller-configured HTTP client.
    #[must_use]
    pub fn http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = Some(http_client);
        self
    }

    /// Validates the configuration and builds the client.
    pub fn build(self) -> Result<Client, Error> {
        let key = self.api_key.expose_secret().trim();
        if key.is_empty() {
            return Err(Error::InvalidConfiguration("api_key is required".into()));
        }
        if key.chars().any(char::is_whitespace) {
            return Err(Error::InvalidConfiguration(
                "api_key must not contain whitespace".into(),
            ));
        }

        let mut authorization = HeaderValue::from_str(&format!("Bearer {key}")).map_err(|_| {
            Error::InvalidConfiguration("api_key is not a valid header value".into())
        })?;
        authorization.set_sensitive(true);

        let mut base_url = Url::parse(&self.base_url)
            .map_err(|error| Error::InvalidConfiguration(format!("invalid base_url: {error}")))?;
        if base_url.query().is_some() || base_url.fragment().is_some() {
            return Err(Error::InvalidConfiguration(
                "base_url must not contain a query or fragment".into(),
            ));
        }
        let host = base_url.host_str().unwrap_or_default();
        let secure = base_url.scheme() == "https";
        let local_http =
            base_url.scheme() == "http" && matches!(host, "localhost" | "127.0.0.1" | "::1");
        if !secure && !local_http {
            return Err(Error::InvalidConfiguration(
                "base_url must use HTTPS unless it targets a loopback host".into(),
            ));
        }
        if !base_url.path().ends_with('/') {
            let path = format!("{}/", base_url.path());
            base_url.set_path(&path);
        }

        Ok(Client {
            http: self.http_client.unwrap_or_default(),
            base_url,
            authorization,
            retry_policy: self.retry_policy,
        })
    }
}

/// Async client for the Heyrafiki API.
#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: Url,
    authorization: HeaderValue,
    retry_policy: RetryPolicy,
}

impl Client {
    /// Creates a client using the production API base URL.
    pub fn new(api_key: impl Into<String>) -> Result<Self, Error> {
        Self::builder(api_key).build()
    }

    /// Starts a client builder.
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder {
            api_key: SecretString::from(api_key.into()),
            base_url: DEFAULT_BASE_URL.into(),
            retry_policy: RetryPolicy::default(),
            http_client: None,
        }
    }

    pub(crate) fn endpoint(&self, segments: &[&str]) -> Result<Url, Error> {
        let mut url = self.base_url.clone();
        {
            let mut path = url.path_segments_mut().map_err(|()| {
                Error::InvalidConfiguration("base_url cannot accept path segments".into())
            })?;
            path.pop_if_empty();
            for segment in segments {
                path.push(segment);
            }
        }
        Ok(url)
    }

    pub(crate) async fn get<T>(&self, url: Url) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.execute(RequestSpec::new(Method::GET, url).retriable())
            .await
    }

    pub(crate) async fn post_json<T, B>(
        &self,
        url: Url,
        body: &B,
        headers: HeaderMap,
        retriable: bool,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let mut spec = RequestSpec::new(Method::POST, url)
            .headers(headers)
            .body(serde_json::to_vec(body)?);
        if retriable {
            spec = spec.retriable();
        }
        self.execute(spec).await
    }

    pub(crate) async fn post_empty<T>(&self, url: Url) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.execute(RequestSpec::new(Method::POST, url)).await
    }

    pub(crate) async fn delete<T>(&self, url: Url) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.execute(RequestSpec::new(Method::DELETE, url)).await
    }

    async fn execute<T>(&self, spec: RequestSpec) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        for attempt in 0..=self.retry_policy.max_retries {
            let mut request = self
                .http
                .request(spec.method.clone(), spec.url.clone())
                .header(ACCEPT, "application/json")
                .header(AUTHORIZATION, self.authorization.clone())
                .headers(spec.headers.clone());
            if let Some(body) = &spec.body {
                request = request
                    .header(CONTENT_TYPE, "application/json")
                    .body(body.clone());
            }

            let response = request.send().await?;
            let status = response.status();
            let request_id = header_text(response.headers(), "x-request-id");
            let retry_after = parse_retry_after(response.headers().get(RETRY_AFTER));
            let bytes = response.bytes().await?;

            if status.is_success() {
                return serde_json::from_slice(&bytes).map_err(|source| Error::InvalidResponse {
                    status,
                    request_id,
                    source,
                });
            }

            let api_error = parse_api_error(status, request_id, retry_after, &bytes);
            if spec.retriable
                && matches!(
                    status,
                    StatusCode::TOO_MANY_REQUESTS | StatusCode::SERVICE_UNAVAILABLE
                )
                && attempt < self.retry_policy.max_retries
                && let Some(delay) = self.retry_delay(attempt, retry_after)
            {
                tokio::time::sleep(delay).await;
                continue;
            }
            return Err(Error::Api(api_error));
        }
        unreachable!("retry loop always returns")
    }

    fn retry_delay(&self, attempt: u8, retry_after: Option<Duration>) -> Option<Duration> {
        if let Some(delay) = retry_after {
            return (delay <= self.retry_policy.max_delay).then_some(delay);
        }
        let multiplier = 1_u32.checked_shl(u32::from(attempt)).unwrap_or(u32::MAX);
        let base = self.retry_policy.base_delay.saturating_mul(multiplier);
        let jitter_upper = u64::try_from(base.as_millis()).unwrap_or(u64::MAX);
        let jitter = Duration::from_millis(fastrand::u64(0..=jitter_upper));
        Some(base.saturating_add(jitter).min(self.retry_policy.max_delay))
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Client")
            .field("base_url", &self.base_url)
            .field("retry_policy", &self.retry_policy)
            .finish_non_exhaustive()
    }
}

#[derive(Clone)]
struct RequestSpec {
    method: Method,
    url: Url,
    headers: HeaderMap,
    body: Option<Vec<u8>>,
    retriable: bool,
}

impl RequestSpec {
    fn new(method: Method, url: Url) -> Self {
        Self {
            method,
            url,
            headers: HeaderMap::new(),
            body: None,
            retriable: false,
        }
    }

    fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    const fn retriable(mut self) -> Self {
        self.retriable = true;
        self
    }
}

fn parse_api_error(
    status: StatusCode,
    request_id: Option<String>,
    retry_after: Option<Duration>,
    bytes: &[u8],
) -> ApiError {
    let envelope = serde_json::from_slice::<ErrorEnvelope>(bytes).ok();
    ApiError {
        status,
        code: envelope
            .as_ref()
            .map_or_else(|| "request_failed".into(), |value| value.error.code.clone()),
        message: envelope.as_ref().map_or_else(
            || "The Heyrafiki API request failed.".into(),
            |value| value.error.message.clone(),
        ),
        request_id,
        docs: envelope.map(|value| value.error.docs),
        retry_after,
    }
}

fn header_text(headers: &HeaderMap, name: &'static str) -> Option<String> {
    headers.get(name)?.to_str().ok().map(str::to_owned)
}

fn parse_retry_after(value: Option<&HeaderValue>) -> Option<Duration> {
    let value = value?.to_str().ok()?;
    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }
    let date = httpdate::parse_http_date(value).ok()?;
    Some(date.duration_since(SystemTime::now()).unwrap_or_default())
}

pub(crate) fn idempotency_headers(options: &WriteOptions) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("idempotency-key", options.idempotency_key.clone());
    headers
}
