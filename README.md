# Heyrafiki Rust SDK

Async Rust client for the Heyrafiki API.

## What this SDK is for

Use this SDK to call the Heyrafiki API from a Rust service. It provides typed
requests, predictable errors and idempotency support for retried writes. The
platform applies access rules; clinical and financial decisions remain with the
accountable people and organizations.

The client implements the operations published in the Heyrafiki OpenAPI 3.1
contract. It provides typed requests, bounded retries, idempotency and stable
errors, while the platform keeps access rules and decision-making in one place.

## Build from source

Clone the public source and use a path dependency:

```bash
git clone https://github.com/heyrafiki/rafiki-rs.git
```

```toml
[dependencies]
heyrafiki = { path = "../rafiki-rs" }
```

Rust 1.85 or newer is required. Keep API keys on the server.

## First request

```rust,no_run
use heyrafiki::{Client, ListParams};

#[tokio::main]
async fn main() -> Result<(), heyrafiki::Error> {
    let client = Client::new(std::env::var("HEYRAFIKI_API_KEY").expect("API key"))?;
    let practitioners = client
        .practitioners()
        .list(ListParams::new(5)?)
        .await?;

    println!("{}", practitioners.data.len());
    Ok(())
}
```

Sandbox keys return synthetic data.

## Idempotent writes

Operations that require `Idempotency-Key` accept a validated `WriteOptions`.
The client automatically retries `429` and `503` only for reads and writes
that carry an idempotency key.

```rust,no_run
use heyrafiki::{BookingInput, CareFormat, Client, PaymentSource, WriteOptions};

# async fn run(client: &Client) -> Result<(), heyrafiki::Error> {
let booking = client
    .bookings()
    .create(
        &BookingInput {
            practitioner_id: "prc_2481".into(),
            starts_at: "2026-08-10T07:00:00Z".into(),
            ends_at: "2026-08-10T08:00:00Z".into(),
            format: CareFormat::Online,
            payment_source: PaymentSource::SelfPay,
        },
        WriteOptions::new("booking-demo-0001")?,
    )
    .await?;
# let _ = booking;
# Ok(())
# }
```

Amounts use the currency's minor unit.

## Errors

```rust,no_run
use heyrafiki::{Client, Error};

# async fn run(client: &Client) {
match client.claims().retrieve("clm_123").await {
    Err(Error::Api(error)) => {
        eprintln!("{} {} {:?}", error.status, error.code, error.request_id);
    }
    _ => {}
}
# }
```

Branch on `ApiError::code`. Messages may change. Keep `request_id` when
contacting support.

## Verify locally

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --all-features --no-deps
cargo package
```

## Resources

- [Documentation](https://docs.heyrafiki.space)
- [API contract](https://github.com/heyrafiki/contract)
- [Open insurance assurance benchmark](https://github.com/heyrafiki/proving-ground)
- [Security](https://github.com/heyrafiki/.github/blob/main/SECURITY.md)

## License

Licensed under the [Apache License 2.0](./LICENSE).
