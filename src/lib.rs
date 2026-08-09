//! Async Rust client for the Heyrafiki API.
//!
//! This Source Preview implements the operations in the pinned public `OpenAPI`
//! contract. Domain rules and data authority remain in the Heyrafiki Platform.
//!
//! API keys must remain in server-side environments. Reads and writes carrying
//! an idempotency key retry only `429` and `503` responses.

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod client;
mod error;
mod models;
mod resources;

pub use client::{
    Client, ClientBuilder, CoverageBatchOptions, ListParams, RetryPolicy, WriteOptions,
};
pub use error::{ApiError, Error};
pub use models::*;
pub use resources::{
    Api, Bookings, Claims, CoverageBatches, Coverages, EligibilityChecks, Operation, Practitioners,
    Preauthorizations, Remittances, SUPPORTED_OPERATIONS, Sessions, WebhookEndpoints,
};
