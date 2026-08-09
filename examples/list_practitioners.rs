//! Lists Practitioners with the default pagination settings.

use heyrafiki::{Client, ListParams};

#[tokio::main]
async fn main() -> Result<(), heyrafiki::Error> {
    let api_key = std::env::var("HEYRAFIKI_API_KEY").map_err(|_| {
        heyrafiki::Error::InvalidConfiguration("HEYRAFIKI_API_KEY is required".into())
    })?;
    let client = Client::new(api_key)?;
    let practitioners = client.practitioners().list(ListParams::new(5)?).await?;

    for practitioner in practitioners.data {
        println!("{} ({})", practitioner.name, practitioner.id);
    }
    Ok(())
}
