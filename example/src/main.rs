use lazerpay_rust_sdk::{Lazerpay};

#[tokio::main]
async fn main() {
    let api_public_key = "pk_live_GHRzvJIIobM0Yvpec61tquOToX8jae3SnQEDwrGeCUbY0IzPD6".to_string();

    let api_secret_key: String = "sk_live_GZrPRI0EhqDgjWvKbbJf94lYfN8lBacwNXY2h9iHvClPqXTEQT".to_string();

    let lazerpay: Lazerpay = Lazerpay::new(api_public_key, api_secret_key);

    // Initialize lazerpay payment
    let reference = "W6b8hV55l0435t3545435".to_string();
    let amount = "1000".to_string();
    let customer_name = "Enoch".to_string();
    let customer_email = "enochchejieh@gmail.com".to_string();
    let coin = "USDC".to_string();
    let currency = "USD".to_string();
    let api_public_key = "pk_live_GHRzvJIIobM0Yvpec61tquOToX8jae3SnQEDwrGeCUbY0IzPD6".to_string();
    let accept_partial_payment = true;

    lazerpay.payment.initialize_payment(reference, amount, customer_name, customer_email, coin, currency, api_public_key, accept_partial_payment).await;
}
