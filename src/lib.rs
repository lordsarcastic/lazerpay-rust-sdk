mod constants;
mod payment;
pub mod utils;

pub struct Lazerpay {
    pub api_public_key: String,
    pub api_secret_key: String,
    pub payment: payment::Payment,
}

impl Lazerpay {
    pub fn new(api_public_key: &String, api_secret_key: &String) -> Self {
        Lazerpay {
            api_public_key: api_public_key.to_string(),
            api_secret_key: api_secret_key.to_string(),
            payment: payment::Payment::new(api_public_key.to_string(), api_secret_key.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Lazerpay {
        let api = utils::get_api_keys().unwrap();
        let lazerpay: Lazerpay = Lazerpay::new(&api.public_key, &api.secret_key);
        lazerpay
    }

    #[tokio::test]
    async fn initialize_payment() {
        let api = utils::get_api_keys().unwrap();
        let response = setup()
            .payment
            .initialize_payment(
                "xxxxxxxxxxxxxxxxxxxxx".to_string(),
                "10000".to_string(),
                "xxxxx".to_string(),
                "xxxxxxx@gmail.com".to_string(),
                "USDC".to_string(),
                "USD".to_string(),
                api.public_key.to_string(),
                true,
            )
            .await;

        assert_eq!(response["message"], "Reference already exists");
    }

    #[tokio::test]
    async fn confirm_payment() {
        let response = setup()
            .payment
            .confirm_payment("xxxxxxxxxxxxxxxxxxxxx".to_string())
            .await;

        assert_eq!(response["statusCode"], 200);
    }

    #[tokio::test]
    async fn get_accepted_coins() {
        let response = setup().payment.get_accepted_coins().await;

        assert_eq!(response["statusCode"], 200);
    }

    #[tokio::test]
    async fn get_rate() {
        let response = setup()
            .payment
            .get_rate("USD".to_string(), "USDC".to_string())
            .await;

        assert_eq!(response["statusCode"], 200);
    }

    #[tokio::test]
    async fn transfer_funds() {
        let api = utils::get_api_keys().unwrap();
        let response = setup()
            .payment
            .transfer_funds(
                1000,
                "0x0000000000000000000000000000000000000000".to_string(),
                "USDC".to_string(),
                "Binance Smart Chain".to_string(),
                api.public_key.to_string(),
                api.secret_key.to_string(),
            )
            .await;

        assert_eq!(
            response["message"],
            "Insufficient funds, check your balance and try again"
        );
    }
}
