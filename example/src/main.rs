use serde_json::Value;
use lazerpay_rust_sdk::Lazerpay;
mod constants;

#[tokio::main]
async fn main() {
    let api_public_key = constants::API_PUBLIC_KEY_ENV_VAR.to_string();
    let api_secret_key = constants::API_SECRET_KEY_ENV_VAR.to_string();
    let lazerpay: Lazerpay = Lazerpay::new(&api_public_key, &api_secret_key);

    // Initialize lazerpay payment
    let response = initialize_payment(
        lazerpay, 
        "W6b8hV55l0435t3545435".to_string(),
        "1000".to_string(),
        "Enoch".to_string(),
        "enochchejieh@gmail.com".to_string(),
        "USDC".to_string(),
        "USD".to_string(),
        api_public_key.to_string(),
        true
    ).await;

    println!("{:?}", response);
}

async fn initialize_payment (
    lazerpay: Lazerpay,
    reference: String, 
    amount: String,
    customer_name: String,
    customer_email: String, 
    coin: String,
    currency: String, 
    api_public_key: String, 
    accept_partial_payment: bool
    ) -> Value {
    lazerpay.payment.initialize_payment(
        reference, 
        amount, 
        customer_name, 
        customer_email, 
        coin, 
        currency, 
        api_public_key,
        accept_partial_payment
    ).await
}
