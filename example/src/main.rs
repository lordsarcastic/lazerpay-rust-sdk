use serde_json::Value;
use lazerpay_rust_sdk::Lazerpay;

mod utils;

#[tokio::main]
async fn main() {
    let api = utils::get_api_keys().unwrap();
    let lazerpay: Lazerpay = Lazerpay::new(&api.public_key, &api.secret_key);

    // Initialize lazerpay payment
    let response = initialize_payment(
        lazerpay, 
        "W6b8hV55l0435t3545435".to_string(),
        "1000".to_string(),
        "Enoch".to_string(),
        "enochchejieh@gmail.com".to_string(),
        "USDC".to_string(),
        "USD".to_string(),
        api.public_key.to_string(),
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
