use lazerpay_rust_sdk::Lazerpay;

#[tokio::main]
async fn main() {
    let api_public_key = "123".to_string();
    let api_secret_key: String = "123".to_string();
    let lazerpay: Lazerpay = Lazerpay::new(api_public_key, api_secret_key);

    lazerpay.payment.confirm_payment("*******").await;
}
