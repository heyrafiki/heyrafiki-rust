//! Contract-backed request and response models.

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Environment {
    Sandbox,
    Production,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiVersion {
    #[serde(rename = "v1")]
    V1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    Api,
    List,
    Practitioner,
    Availability,
    Booking,
    Session,
    EligibilityCheck,
    CoverageObservation,
    CoverageBatchResult,
    Preauthorization,
    PreauthorizationDecision,
    Claim,
    ClaimInformationRequest,
    ClaimAdjudication,
    Remittance,
    WebhookEndpoint,
    WebhookDelivery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CareFormat {
    Online,
    InPerson,
    Phone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentSource {
    SelfPay,
    Covered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    Kes,
    Usd,
    Eur,
    Gbp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookingStatus {
    Reserved,
    Confirmed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Reserved,
    Confirmed,
    InProgress,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EligibilityStatus {
    Eligible,
    Ineligible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EligibilityReasonCode {
    Eligible,
    CoverageNotFound,
    CoverageInactive,
    CoordinationRequired,
    OutsideCoveragePeriod,
    BenefitExhausted,
    AmountExceedsBenefit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageStatus {
    Active,
    Paused,
    Exhausted,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageSource {
    PayerApi,
    BatchFile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreauthorizationStatus {
    Pending,
    Approved,
    Denied,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreauthorizationOutcome {
    Approved,
    Denied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimStatus {
    Draft,
    Submitted,
    Queried,
    Approved,
    PartiallyApproved,
    Denied,
    Settled,
    Reversed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimInformationRequestStatus {
    Open,
    Fulfilled,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimDecision {
    Approved,
    PartiallyApproved,
    Denied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemittanceStatus {
    Received,
    Reconciled,
    Exception,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookStatus {
    Active,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebhookEventType {
    #[serde(rename = "sandbox.ping")]
    SandboxPing,
    #[serde(rename = "preauthorization.requested")]
    PreauthorizationRequested,
    #[serde(rename = "preauthorization.approved")]
    PreauthorizationApproved,
    #[serde(rename = "preauthorization.denied")]
    PreauthorizationDenied,
    #[serde(rename = "preauthorization.expired")]
    PreauthorizationExpired,
    #[serde(rename = "claim.submitted")]
    ClaimSubmitted,
    #[serde(rename = "claim.information_requested")]
    ClaimInformationRequested,
    #[serde(rename = "claim.resubmitted")]
    ClaimResubmitted,
    #[serde(rename = "claim.approved")]
    ClaimApproved,
    #[serde(rename = "claim.partially_approved")]
    ClaimPartiallyApproved,
    #[serde(rename = "claim.denied")]
    ClaimDenied,
    #[serde(rename = "claim.settled")]
    ClaimSettled,
    #[serde(rename = "remittance.reconciled")]
    RemittanceReconciled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiInformation {
    pub object: ObjectType,
    pub version: ApiVersion,
    pub environment: Environment,
    pub resources: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct List<T> {
    pub object: ObjectType,
    pub data: Vec<T>,
    pub has_more: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Location {
    pub city: String,
    pub country: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Money {
    pub amount: i64,
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Practitioner {
    pub id: String,
    pub object: ObjectType,
    pub name: String,
    pub profession: String,
    pub location: Location,
    pub session_fee: Money,
}

pub type PractitionerList = List<Practitioner>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeeklyHours {
    pub weekday: u8,
    pub start: String,
    pub end: String,
    pub formats: Vec<CareFormat>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PractitionerAvailability {
    pub object: ObjectType,
    pub practitioner_id: String,
    pub timezone: String,
    pub weekly_hours: Vec<WeeklyHours>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Booking {
    pub id: String,
    pub object: ObjectType,
    pub session_id: String,
    pub practitioner_id: String,
    pub starts_at: String,
    pub ends_at: String,
    pub timezone: String,
    pub format: CareFormat,
    pub status: BookingStatus,
    pub payment_source: PaymentSource,
}

pub type BookingList = List<Booking>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BookingInput {
    pub practitioner_id: String,
    pub starts_at: String,
    pub ends_at: String,
    pub format: CareFormat,
    pub payment_source: PaymentSource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Session {
    pub id: String,
    pub object: ObjectType,
    pub practitioner_id: String,
    pub starts_at: String,
    pub ends_at: String,
    pub timezone: String,
    pub format: CareFormat,
    pub status: SessionStatus,
    pub payment_source: PaymentSource,
}

pub type SessionList = List<Session>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EligibilityCheckInput {
    pub member_reference: String,
    pub service_code: String,
    pub scheduled_at: String,
    pub amount: i64,
    pub currency: Currency,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EligibilityService {
    pub code: String,
    pub scheduled_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EligibilityAmount {
    pub requested: i64,
    pub currency: String,
    pub maximum_per_session: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EligibilityCheck {
    pub id: String,
    pub object: ObjectType,
    pub status: EligibilityStatus,
    pub reason_codes: Vec<EligibilityReasonCode>,
    pub service: EligibilityService,
    pub amount: EligibilityAmount,
    pub authorization_required: bool,
    pub remaining_sessions: Option<i64>,
    pub coverage_valid_until: Option<String>,
    pub checked_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverageObservationInput {
    pub source_contract_reference: String,
    pub external_coverage_reference: String,
    pub source_version: String,
    pub tenant_reference: String,
    pub member_reference: String,
    pub plan_name: String,
    pub service_code: String,
    pub status: CoverageStatus,
    pub currency: Currency,
    pub amount_limit: i64,
    pub remaining_sessions: i64,
    pub authorization_required: bool,
    pub coordination_priority: Option<i64>,
    pub valid_from: String,
    pub valid_until: String,
    pub observed_at: String,
    pub evidence_references: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverageAmountLimit {
    pub currency: Currency,
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverageObservation {
    pub id: String,
    pub object: ObjectType,
    pub coverage_id: String,
    pub source: CoverageSource,
    pub source_contract_reference: String,
    pub external_coverage_reference: String,
    pub source_version: String,
    pub snapshot_version: i64,
    pub status: CoverageStatus,
    pub service_code: String,
    pub amount_limit: CoverageAmountLimit,
    pub remaining_sessions: i64,
    pub authorization_required: bool,
    pub coordination_priority: Option<i64>,
    pub valid_from: String,
    pub valid_until: String,
    pub observed_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverageBatchRecordInput {
    pub coverage_reference: String,
    pub record_version: String,
    pub tenant_reference: String,
    pub member_reference: String,
    pub plan_name: String,
    pub service_code: String,
    pub status: CoverageStatus,
    pub currency: Currency,
    pub amount_limit: i64,
    pub remaining_sessions: i64,
    pub authorization_required: bool,
    pub coordination_priority: Option<i64>,
    pub valid_from: String,
    pub valid_until: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoverageBatchContractVersion {
    #[serde(rename = "2026-08-01")]
    V20260801,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverageBatchInput {
    pub contract_version: CoverageBatchContractVersion,
    pub batch_reference: String,
    pub batch_version: String,
    pub source_contract_reference: String,
    pub generated_at: String,
    pub records: Vec<CoverageBatchRecordInput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverageBatchResult {
    pub object: ObjectType,
    pub batch_reference: String,
    pub artifact_sha256: String,
    pub total: i64,
    pub recorded: i64,
    pub replayed: i64,
    pub observations: Vec<CoverageObservation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreauthorizationInput {
    pub eligibility_check_id: String,
    pub booking_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case", deny_unknown_fields)]
pub enum PreauthorizationDecisionInput {
    Approved {
        approved_amount: i64,
        valid_until: String,
        reason_codes: Vec<String>,
        policy_reference: String,
        policy_version: String,
        evidence_references: Vec<String>,
    },
    Denied {
        reason_codes: Vec<String>,
        policy_reference: String,
        policy_version: String,
        evidence_references: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyReference {
    pub reference: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreauthorizationDecision {
    pub id: String,
    pub object: ObjectType,
    pub version: i64,
    pub outcome: PreauthorizationOutcome,
    pub reason_codes: Vec<String>,
    pub policy: PolicyReference,
    pub authority_reference: String,
    pub evidence_references: Vec<String>,
    pub decided_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreauthorizationAmount {
    pub requested: i64,
    pub approved: Option<i64>,
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Preauthorization {
    pub id: String,
    pub object: ObjectType,
    pub eligibility_check_id: String,
    pub booking_id: String,
    pub status: PreauthorizationStatus,
    pub reason_codes: Vec<String>,
    pub amount: PreauthorizationAmount,
    pub valid_until: Option<String>,
    pub created_at: String,
    pub decision: Option<PreauthorizationDecision>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimLine {
    pub line_number: i64,
    pub code_system: String,
    pub code_system_version: Option<String>,
    pub service_code: String,
    pub units: f64,
    pub amount: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimInputLine {
    pub code_system: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_system_version: Option<String>,
    pub service_code: String,
    pub units: f64,
    pub amount: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimInput {
    pub eligibility_check_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preauthorization_id: Option<String>,
    pub session_id: String,
    pub provider_claim_reference: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_refs: Option<Vec<String>>,
    pub lines: Vec<ClaimInputLine>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimInformationRequest {
    pub id: String,
    pub object: ObjectType,
    pub reason_code: String,
    pub requested_evidence_types: Vec<String>,
    pub status: ClaimInformationRequestStatus,
    pub due_at: Option<String>,
    pub created_at: String,
    pub resolved_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimInformationRequestInput {
    pub reason_code: String,
    pub requested_evidence_types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimEvidenceInput {
    pub information_request_id: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimAdjudicationLineAmount {
    pub billed: i64,
    pub allowed: i64,
    pub payer: i64,
    pub patient_responsibility: i64,
    pub adjustment: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimAdjudicationLine {
    pub line_number: i64,
    pub amount: ClaimAdjudicationLineAmount,
    pub reason_codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimAdjudicationInput {
    pub policy_reference: String,
    pub policy_version: String,
    pub reason_codes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_refs: Option<Vec<String>>,
    pub lines: Vec<ClaimAdjudicationLine>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimAmount {
    pub currency: String,
    pub billed: i64,
    pub approved: Option<i64>,
    pub remitted: Option<i64>,
    pub settled: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServicePeriod {
    pub starts_at: String,
    pub ends_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimAdjudicationAmount {
    pub currency: String,
    pub billed: i64,
    pub payer: i64,
    pub patient_responsibility: i64,
    pub adjustment: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimAdjudication {
    pub id: String,
    pub object: ObjectType,
    pub version: i64,
    pub decision: ClaimDecision,
    pub amount: ClaimAdjudicationAmount,
    pub reason_codes: Vec<String>,
    pub policy: PolicyReference,
    pub lines: Vec<ClaimAdjudicationLine>,
    pub decided_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Claim {
    pub id: String,
    pub object: ObjectType,
    pub status: ClaimStatus,
    pub provider_claim_reference: Option<String>,
    pub submission_version: i64,
    pub amount: ClaimAmount,
    pub service_period: ServicePeriod,
    pub lines: Vec<ClaimLine>,
    pub information_requests: Vec<ClaimInformationRequest>,
    pub adjudication: Option<ClaimAdjudication>,
    pub submitted_at: Option<String>,
    pub updated_at: String,
}

pub type ClaimList = List<Claim>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemittanceAllocation {
    pub claim_id: String,
    pub paid_amount: i64,
    pub reason_codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemittanceAllocationInput {
    pub claim_id: String,
    pub paid_amount: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemittanceInput {
    pub payer_reference: String,
    pub currency: Currency,
    pub received_at: String,
    pub allocations: Vec<RemittanceAllocationInput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemittanceAmount {
    pub currency: String,
    pub paid: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Remittance {
    pub id: String,
    pub object: ObjectType,
    pub status: RemittanceStatus,
    pub payer_reference: String,
    pub amount: RemittanceAmount,
    pub allocations: Vec<RemittanceAllocation>,
    pub received_at: String,
    pub reconciled_at: Option<String>,
}

pub type RemittanceList = List<Remittance>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebhookEndpointInput {
    pub url: String,
    pub events: Vec<WebhookEventType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebhookEndpoint {
    pub id: String,
    pub object: ObjectType,
    pub url: String,
    pub events: Vec<String>,
    pub status: WebhookStatus,
    pub created_at: String,
    pub disabled_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebhookEndpointWithSecret {
    pub id: String,
    pub object: ObjectType,
    pub url: String,
    pub events: Vec<String>,
    pub status: WebhookStatus,
    pub created_at: String,
    pub disabled_at: Option<String>,
    pub signing_secret: String,
}

pub type WebhookEndpointList = List<WebhookEndpoint>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebhookDelivery {
    pub id: String,
    pub object: ObjectType,
    pub delivered: bool,
    pub attempts: i64,
    pub response_status: Option<u16>,
}
