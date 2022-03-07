use anyhow::Result;
use lazerpay_rust_sdk::{utils, Customer, Lazerpay, PaymentConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let api = Lazerpay::new()?;

    let mut payment = api.create_payment(PaymentConfig {
        customer: Customer::new("Enoch", "enochchejieh@gmail.com"),
        reference: utils::generate_unique_reference(),
        coin: "USDC".into(),
        amount: "1000".into(),
        currency: "USD".into(),
        accept_partial_payment: true,
    });

    dbg!(payment.send().await?);

    dbg!(payment.check_confirmation().await?);

    Ok(())
}
