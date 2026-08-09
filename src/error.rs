//! Error types returned by the client.

use std::{fmt, time::Duration};

use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

/// An error returned by the Heyrafiki API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    /// HTTP status returned by the API.
    pub status: StatusCode,
    /// Stable API error code.
    pub code: String,
    /// Human-readable message. Do not branch on this field.
    pub message: String,
    /// Request identifier for support and tracing.
    pub request_id: Option<String>,
    /// Documentation URL supplied by the API.
    pub docs: Option<String>,
    /// Server-requested delay, when supplied.
    pub retry_after: Option<Duration>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Heyrafiki API error {}: {}",
            self.status, self.code
        )
    }
}

impl std::error::Error for ApiError {}

/// Errors produced while configuring or calling the client.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The API returned a structured failure.
    #[error(transparent)]
    Api(#[from] ApiError),
    /// The HTTP request could not be completed.
    #[error("Heyrafiki request failed: {0}")]
    Transport(#[from] reqwest::Error),
    /// A typed request could not be serialized.
    #[error("could not serialize the Heyrafiki request: {0}")]
    Serialization(#[from] serde_json::Error),
    /// A successful response did not match the published contract.
    #[error("Heyrafiki returned an invalid response (status {status})")]
    InvalidResponse {
        /// HTTP status returned by the API.
        status: StatusCode,
        /// Request identifier, when present.
        request_id: Option<String>,
        /// JSON decoding failure.
        #[source]
        source: serde_json::Error,
    },
    /// Client configuration is invalid.
    #[error("invalid Heyrafiki client configuration: {0}")]
    InvalidConfiguration(String),
    /// A caller-provided operation value violates the public contract.
    #[error("invalid {field}: {reason}")]
    InvalidInput {
        /// Input field name.
        field: &'static str,
        /// Contract requirement that was not met.
        reason: &'static str,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct ErrorEnvelope {
    pub error: ErrorBody,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ErrorBody {
    pub code: String,
    pub message: String,
    pub docs: String,
}
