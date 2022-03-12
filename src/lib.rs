mod api;
mod constants;
mod errors;
mod response;
pub mod utils;

pub use api::*;
pub use errors::*;
pub use response::*;

#[cfg(test)]
mod tests {
    use crate::PaymentConfig;

    use super::*;

    #[tokio::test]
    async fn send_payment_successful() {
        let reference = &utils::generate_unique_reference();
        let api = Lazerpay::new().unwrap();
        let mut payment = api.create_payment(PaymentConfig {
            customer: Customer::new("Lagbaja Tamedun", "lagbaja.tamedun@gmail.com"),
            reference: reference.clone(),
            coin: "USDC".into(),
            amount: "1000".into(),
            currency: "USD".into(),
            accept_partial_payment: true,
        });

        let response = payment.send().await.unwrap();
        let data = response.data.as_ref().unwrap();

        assert_eq!(response.status, Some(String::from("success")));
        assert_eq!(data.currency, "USD");
        assert_eq!(data.coin, "USDC");
        assert_eq!(data.reference, *reference);
    }

    #[tokio::test]
    async fn confirm_payment_successful() {
        let reference = &utils::generate_unique_reference();
        let api = Lazerpay::new().unwrap();
        let mut payment = api.create_payment(PaymentConfig {
            customer: Customer::new("Lagbaja Tamedun", "lagbaja.tamedun@gmail.com"),
            reference: reference.clone(),
            coin: "USDC".into(),
            amount: "1000".into(),
            currency: "USD".into(),
            accept_partial_payment: true,
        });

        let response = {
            let _ = payment.send().await.unwrap();
            payment.check_confirmation().await.unwrap()
        };

        let data = response.data.as_ref().unwrap();

        assert_eq!(response.status, Some(String::from("success")));
        assert_eq!(response.message, "Verification successful");
        assert_eq!(data.currency, "USD");
        assert_eq!(data.coin, "USDC");
    }

    #[tokio::test]
    async fn send_transfer_insufficient() {
        let api = Lazerpay::new().unwrap();
        let transfer = api.create_transfer(TransferConfig {
            amount: 1000,
            recipient: "0x0000000000000000000000000000000000000000".into(),
            coin: "USDC".into(),
            blockchain: "Binance Smart Chain".into(),
        });

        let response = transfer.send().await.unwrap();

        assert_eq!(
            response.message,
            "Insufficient funds, check your balance and try again"
        );
    }

    #[tokio::test]
    async fn get_accepted_coins_successful() {
        let api = Lazerpay::new().unwrap();
        let response = api.get_accepted_coins().await.unwrap();
        let data = response.data.as_ref().unwrap();

        assert_eq!(response.status, Some(String::from("success")));
        assert!(data.len() > 0);
    }

    // TODO(appcypher): This API is currently not working as expected.
    // #[tokio::test]
    // async fn get_rate_successful() {
    //     let api = Lazerpay::new();
    //     let _response = api.get_rate("NGN", "USDT").await.unwrap();

    //     assert_eq!(response.status, Some(String::from("success")));
    //     assert!(response.rate > 0.0);
    // }
}
