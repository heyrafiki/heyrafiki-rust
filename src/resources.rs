//! Resource-scoped API clients.

use reqwest::header::HeaderMap;

use crate::{
    Client, CoverageBatchOptions, ListParams, WriteOptions,
    client::idempotency_headers,
    error::Error,
    models::{
        ApiInformation, Booking, BookingInput, BookingList, Claim, ClaimAdjudicationInput,
        ClaimEvidenceInput, ClaimInformationRequestInput, ClaimList, CoverageBatchInput,
        CoverageBatchResult, CoverageObservation, CoverageObservationInput, EligibilityCheck,
        EligibilityCheckInput, Practitioner, PractitionerAvailability, PractitionerList,
        Preauthorization, PreauthorizationDecisionInput, PreauthorizationInput, Remittance,
        RemittanceInput, RemittanceList, Session, SessionList, WebhookDelivery, WebhookEndpoint,
        WebhookEndpointInput, WebhookEndpointList, WebhookEndpointWithSecret,
    },
};

/// A published `OpenAPI` operation implemented by this SDK.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Operation {
    /// `OpenAPI` operation identifier.
    pub operation_id: &'static str,
    /// HTTP method.
    pub method: &'static str,
    /// `OpenAPI` path template.
    pub path: &'static str,
}

/// Contract operations implemented by this SDK.
pub const SUPPORTED_OPERATIONS: &[Operation] = &[
    Operation {
        operation_id: "getApi",
        method: "GET",
        path: "/",
    },
    Operation {
        operation_id: "listPractitioners",
        method: "GET",
        path: "/practitioners",
    },
    Operation {
        operation_id: "getPractitioner",
        method: "GET",
        path: "/practitioners/{practitioner_id}",
    },
    Operation {
        operation_id: "getPractitionerAvailability",
        method: "GET",
        path: "/practitioners/{practitioner_id}/availability",
    },
    Operation {
        operation_id: "listBookings",
        method: "GET",
        path: "/bookings",
    },
    Operation {
        operation_id: "createBooking",
        method: "POST",
        path: "/bookings",
    },
    Operation {
        operation_id: "getBooking",
        method: "GET",
        path: "/bookings/{booking_id}",
    },
    Operation {
        operation_id: "listSessions",
        method: "GET",
        path: "/sessions",
    },
    Operation {
        operation_id: "getSession",
        method: "GET",
        path: "/sessions/{session_id}",
    },
    Operation {
        operation_id: "createEligibilityCheck",
        method: "POST",
        path: "/eligibility_checks",
    },
    Operation {
        operation_id: "getEligibilityCheck",
        method: "GET",
        path: "/eligibility_checks/{eligibility_check_id}",
    },
    Operation {
        operation_id: "recordCoverageObservation",
        method: "POST",
        path: "/coverages",
    },
    Operation {
        operation_id: "recordCoverageBatch",
        method: "POST",
        path: "/coverage_batches",
    },
    Operation {
        operation_id: "createPreauthorization",
        method: "POST",
        path: "/preauthorizations",
    },
    Operation {
        operation_id: "getPreauthorization",
        method: "GET",
        path: "/preauthorizations/{preauthorization_id}",
    },
    Operation {
        operation_id: "decidePreauthorization",
        method: "POST",
        path: "/preauthorizations/{preauthorization_id}/decisions",
    },
    Operation {
        operation_id: "listClaims",
        method: "GET",
        path: "/claims",
    },
    Operation {
        operation_id: "createClaim",
        method: "POST",
        path: "/claims",
    },
    Operation {
        operation_id: "getClaim",
        method: "GET",
        path: "/claims/{claim_id}",
    },
    Operation {
        operation_id: "createClaimInformationRequest",
        method: "POST",
        path: "/claims/{claim_id}/information_requests",
    },
    Operation {
        operation_id: "submitClaimEvidence",
        method: "POST",
        path: "/claims/{claim_id}/evidence",
    },
    Operation {
        operation_id: "adjudicateClaim",
        method: "POST",
        path: "/claims/{claim_id}/adjudications",
    },
    Operation {
        operation_id: "listRemittances",
        method: "GET",
        path: "/remittances",
    },
    Operation {
        operation_id: "createRemittance",
        method: "POST",
        path: "/remittances",
    },
    Operation {
        operation_id: "getRemittance",
        method: "GET",
        path: "/remittances/{remittance_id}",
    },
    Operation {
        operation_id: "listWebhookEndpoints",
        method: "GET",
        path: "/webhook_endpoints",
    },
    Operation {
        operation_id: "createWebhookEndpoint",
        method: "POST",
        path: "/webhook_endpoints",
    },
    Operation {
        operation_id: "getWebhookEndpoint",
        method: "GET",
        path: "/webhook_endpoints/{webhook_endpoint_id}",
    },
    Operation {
        operation_id: "disableWebhookEndpoint",
        method: "DELETE",
        path: "/webhook_endpoints/{webhook_endpoint_id}",
    },
    Operation {
        operation_id: "testWebhookEndpoint",
        method: "POST",
        path: "/webhook_endpoints/{webhook_endpoint_id}/test",
    },
];

impl Client {
    /// API discovery operations.
    pub fn api(&self) -> Api<'_> {
        Api { client: self }
    }
    /// Practitioner discovery operations.
    pub fn practitioners(&self) -> Practitioners<'_> {
        Practitioners { client: self }
    }
    /// Booking operations.
    pub fn bookings(&self) -> Bookings<'_> {
        Bookings { client: self }
    }
    /// Session operations.
    pub fn sessions(&self) -> Sessions<'_> {
        Sessions { client: self }
    }
    /// Benefit eligibility operations.
    pub fn eligibility_checks(&self) -> EligibilityChecks<'_> {
        EligibilityChecks { client: self }
    }
    /// Coverage observation operations.
    pub fn coverages(&self) -> Coverages<'_> {
        Coverages { client: self }
    }
    /// Coverage batch operations.
    pub fn coverage_batches(&self) -> CoverageBatches<'_> {
        CoverageBatches { client: self }
    }
    /// Pre-authorization operations.
    pub fn preauthorizations(&self) -> Preauthorizations<'_> {
        Preauthorizations { client: self }
    }
    /// Claim operations.
    pub fn claims(&self) -> Claims<'_> {
        Claims { client: self }
    }
    /// Remittance operations.
    pub fn remittances(&self) -> Remittances<'_> {
        Remittances { client: self }
    }
    /// Webhook endpoint operations.
    pub fn webhook_endpoints(&self) -> WebhookEndpoints<'_> {
        WebhookEndpoints { client: self }
    }
}

/// API discovery client.
pub struct Api<'a> {
    client: &'a Client,
}

impl Api<'_> {
    /// Retrieves API version and environment information.
    pub async fn retrieve(&self) -> Result<ApiInformation, Error> {
        self.client.get(self.client.endpoint(&[])?).await
    }
}

/// Practitioner discovery client.
pub struct Practitioners<'a> {
    client: &'a Client,
}

impl Practitioners<'_> {
    /// Lists Practitioners.
    pub async fn list(&self, params: ListParams) -> Result<PractitionerList, Error> {
        let mut url = self.client.endpoint(&["practitioners"])?;
        params.apply(&mut url);
        self.client.get(url).await
    }

    /// Retrieves a Practitioner.
    pub async fn retrieve(&self, practitioner_id: &str) -> Result<Practitioner, Error> {
        self.client
            .get(self.client.endpoint(&["practitioners", practitioner_id])?)
            .await
    }

    /// Retrieves a Practitioner's availability.
    pub async fn availability(
        &self,
        practitioner_id: &str,
    ) -> Result<PractitionerAvailability, Error> {
        self.client
            .get(
                self.client
                    .endpoint(&["practitioners", practitioner_id, "availability"])?,
            )
            .await
    }
}

/// Booking client.
pub struct Bookings<'a> {
    client: &'a Client,
}

impl Bookings<'_> {
    /// Lists Bookings.
    pub async fn list(&self, params: ListParams) -> Result<BookingList, Error> {
        let mut url = self.client.endpoint(&["bookings"])?;
        params.apply(&mut url);
        self.client.get(url).await
    }

    /// Creates a Booking with a caller-owned idempotency key.
    pub async fn create(
        &self,
        input: &BookingInput,
        options: WriteOptions,
    ) -> Result<Booking, Error> {
        self.client
            .post_json(
                self.client.endpoint(&["bookings"])?,
                input,
                idempotency_headers(&options),
                true,
            )
            .await
    }

    /// Retrieves a Booking.
    pub async fn retrieve(&self, booking_id: &str) -> Result<Booking, Error> {
        self.client
            .get(self.client.endpoint(&["bookings", booking_id])?)
            .await
    }
}

/// Session client.
pub struct Sessions<'a> {
    client: &'a Client,
}

impl Sessions<'_> {
    /// Lists Sessions.
    pub async fn list(&self, params: ListParams) -> Result<SessionList, Error> {
        let mut url = self.client.endpoint(&["sessions"])?;
        params.apply(&mut url);
        self.client.get(url).await
    }

    /// Retrieves a Session.
    pub async fn retrieve(&self, session_id: &str) -> Result<Session, Error> {
        self.client
            .get(self.client.endpoint(&["sessions", session_id])?)
            .await
    }
}

/// Benefit eligibility client.
pub struct EligibilityChecks<'a> {
    client: &'a Client,
}

impl EligibilityChecks<'_> {
    /// Creates an eligibility check with a caller-owned idempotency key.
    pub async fn create(
        &self,
        input: &EligibilityCheckInput,
        options: WriteOptions,
    ) -> Result<EligibilityCheck, Error> {
        self.client
            .post_json(
                self.client.endpoint(&["eligibility_checks"])?,
                input,
                idempotency_headers(&options),
                true,
            )
            .await
    }

    /// Retrieves an eligibility check.
    pub async fn retrieve(&self, eligibility_check_id: &str) -> Result<EligibilityCheck, Error> {
        self.client
            .get(
                self.client
                    .endpoint(&["eligibility_checks", eligibility_check_id])?,
            )
            .await
    }
}

/// Coverage observation client.
pub struct Coverages<'a> {
    client: &'a Client,
}

impl Coverages<'_> {
    /// Records a coverage observation with a caller-owned idempotency key.
    pub async fn record(
        &self,
        input: &CoverageObservationInput,
        options: WriteOptions,
    ) -> Result<CoverageObservation, Error> {
        self.client
            .post_json(
                self.client.endpoint(&["coverages"])?,
                input,
                idempotency_headers(&options),
                true,
            )
            .await
    }
}

/// Coverage batch client.
pub struct CoverageBatches<'a> {
    client: &'a Client,
}

impl CoverageBatches<'_> {
    /// Records a versioned coverage batch.
    pub async fn record(
        &self,
        input: &CoverageBatchInput,
        options: CoverageBatchOptions,
    ) -> Result<CoverageBatchResult, Error> {
        let mut headers = idempotency_headers(&options.write);
        headers.insert("x-heyrafiki-artifact-reference", options.artifact_reference);
        self.client
            .post_json(
                self.client.endpoint(&["coverage_batches"])?,
                input,
                headers,
                true,
            )
            .await
    }
}

/// Pre-authorization client.
pub struct Preauthorizations<'a> {
    client: &'a Client,
}

impl Preauthorizations<'_> {
    /// Creates a pre-authorization with a caller-owned idempotency key.
    pub async fn create(
        &self,
        input: &PreauthorizationInput,
        options: WriteOptions,
    ) -> Result<Preauthorization, Error> {
        self.client
            .post_json(
                self.client.endpoint(&["preauthorizations"])?,
                input,
                idempotency_headers(&options),
                true,
            )
            .await
    }

    /// Retrieves a pre-authorization.
    pub async fn retrieve(&self, preauthorization_id: &str) -> Result<Preauthorization, Error> {
        self.client
            .get(
                self.client
                    .endpoint(&["preauthorizations", preauthorization_id])?,
            )
            .await
    }

    /// Records an accountable pre-authorization decision.
    pub async fn decide(
        &self,
        preauthorization_id: &str,
        input: &PreauthorizationDecisionInput,
        options: WriteOptions,
    ) -> Result<Preauthorization, Error> {
        self.client
            .post_json(
                self.client
                    .endpoint(&["preauthorizations", preauthorization_id, "decisions"])?,
                input,
                idempotency_headers(&options),
                true,
            )
            .await
    }
}

/// Claim client.
pub struct Claims<'a> {
    client: &'a Client,
}

impl Claims<'_> {
    /// Lists Claims.
    pub async fn list(&self, params: ListParams) -> Result<ClaimList, Error> {
        let mut url = self.client.endpoint(&["claims"])?;
        params.apply(&mut url);
        self.client.get(url).await
    }

    /// Creates a Claim with a caller-owned idempotency key.
    pub async fn create(
        &self,
        input: &crate::models::ClaimInput,
        options: WriteOptions,
    ) -> Result<Claim, Error> {
        self.client
            .post_json(
                self.client.endpoint(&["claims"])?,
                input,
                idempotency_headers(&options),
                true,
            )
            .await
    }

    /// Retrieves a Claim.
    pub async fn retrieve(&self, claim_id: &str) -> Result<Claim, Error> {
        self.client
            .get(self.client.endpoint(&["claims", claim_id])?)
            .await
    }

    /// Creates an information request for a Claim.
    pub async fn request_information(
        &self,
        claim_id: &str,
        input: &ClaimInformationRequestInput,
        options: WriteOptions,
    ) -> Result<Claim, Error> {
        self.client
            .post_json(
                self.client
                    .endpoint(&["claims", claim_id, "information_requests"])?,
                input,
                idempotency_headers(&options),
                true,
            )
            .await
    }

    /// Submits evidence for a Claim information request.
    pub async fn submit_evidence(
        &self,
        claim_id: &str,
        input: &ClaimEvidenceInput,
        options: WriteOptions,
    ) -> Result<Claim, Error> {
        self.client
            .post_json(
                self.client.endpoint(&["claims", claim_id, "evidence"])?,
                input,
                idempotency_headers(&options),
                true,
            )
            .await
    }

    /// Records an accountable Claim adjudication.
    pub async fn adjudicate(
        &self,
        claim_id: &str,
        input: &ClaimAdjudicationInput,
        options: WriteOptions,
    ) -> Result<Claim, Error> {
        self.client
            .post_json(
                self.client
                    .endpoint(&["claims", claim_id, "adjudications"])?,
                input,
                idempotency_headers(&options),
                true,
            )
            .await
    }
}

/// Remittance client.
pub struct Remittances<'a> {
    client: &'a Client,
}

impl Remittances<'_> {
    /// Lists remittances.
    pub async fn list(&self, params: ListParams) -> Result<RemittanceList, Error> {
        let mut url = self.client.endpoint(&["remittances"])?;
        params.apply(&mut url);
        self.client.get(url).await
    }

    /// Creates a remittance with a caller-owned idempotency key.
    pub async fn create(
        &self,
        input: &RemittanceInput,
        options: WriteOptions,
    ) -> Result<Remittance, Error> {
        self.client
            .post_json(
                self.client.endpoint(&["remittances"])?,
                input,
                idempotency_headers(&options),
                true,
            )
            .await
    }

    /// Retrieves a remittance.
    pub async fn retrieve(&self, remittance_id: &str) -> Result<Remittance, Error> {
        self.client
            .get(self.client.endpoint(&["remittances", remittance_id])?)
            .await
    }
}

/// Webhook endpoint client.
pub struct WebhookEndpoints<'a> {
    client: &'a Client,
}

impl WebhookEndpoints<'_> {
    /// Lists Webhook endpoints.
    pub async fn list(&self, params: ListParams) -> Result<WebhookEndpointList, Error> {
        let mut url = self.client.endpoint(&["webhook_endpoints"])?;
        params.apply(&mut url);
        self.client.get(url).await
    }

    /// Creates a Webhook endpoint. This operation is not automatically retried.
    pub async fn create(
        &self,
        input: &WebhookEndpointInput,
    ) -> Result<WebhookEndpointWithSecret, Error> {
        self.client
            .post_json(
                self.client.endpoint(&["webhook_endpoints"])?,
                input,
                HeaderMap::new(),
                false,
            )
            .await
    }

    /// Retrieves a Webhook endpoint.
    pub async fn retrieve(&self, endpoint_id: &str) -> Result<WebhookEndpoint, Error> {
        self.client
            .get(self.client.endpoint(&["webhook_endpoints", endpoint_id])?)
            .await
    }

    /// Disables a Webhook endpoint. This operation is not automatically retried.
    pub async fn disable(&self, endpoint_id: &str) -> Result<WebhookEndpoint, Error> {
        self.client
            .delete(self.client.endpoint(&["webhook_endpoints", endpoint_id])?)
            .await
    }

    /// Sends a synthetic test event. This operation is not automatically retried.
    pub async fn send_test(&self, endpoint_id: &str) -> Result<WebhookDelivery, Error> {
        self.client
            .post_empty(
                self.client
                    .endpoint(&["webhook_endpoints", endpoint_id, "test"])?,
            )
            .await
    }
}
