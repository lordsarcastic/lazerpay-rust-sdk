use reqwest::{Client, Response, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use crate::apis::{*};

pub struct InitializePaymentData  {
    pub reference: Option<String>,
    pub amount: String,
    pub customer_name: String,
    pub customer_email: String,
    pub coin: String,
    pub currency: String,
    pub api_public_key: Option<String>,
    pub accept_partial_payment: bool
}

pub struct TransferData  {
    pub amount: u32,
    pub recipient: String,
    pub coin: String,
    pub blockchain: String,
    pub api_public_key: Option<String>,
    pub api_secret_key: Option<String>,
}

pub struct Payment {
    pub api_public_key: String,
    pub api_secret_key: String,
}

impl Payment {
    pub fn new (api_public_key: String, api_secret_key: String) -> Self {
        Payment {
            api_public_key: api_public_key,
            api_secret_key: api_secret_key,
        }
    }

    pub async fn initialize_payment(&self, initialize_payment_data: InitializePaymentData) -> Response {
        let payload = InitializePaymentData {
            reference: initialize_payment_data.reference,
            amount: initialize_payment_data.amount,
            customer_name: initialize_payment_data.customer_name,
            customer_email: initialize_payment_data.customer_email,
            coin: initialize_payment_data.coin,
            currency: initialize_payment_data.currency,
            api_public_key: match initialize_payment_data.api_public_key {
                Some(api_public_key) => Some(api_public_key),
                None => Some(self.api_public_key)
            },
            accept_partial_payment: initialize_payment_data.accept_partial_payment
        };
        let client = Client::new();
        let response = client.post(API_URL_INIT_TRANSACTION)
            .body(&payload)
            .headers(self.construct_headers(true))
            .send()
            .await.unwrap();

        response
    }

    pub async fn confirm_payment (&self, identifier: String) -> Response {
        let client = Client::new();
        let response = client.get(format!("{}/{}",API_URL_CONFIRM_TRANSACTION, identifier))
            .headers(self.construct_headers(false))
            .send()
            .await.unwrap();

        response
    }

    pub async fn get_accepted_coins (&self) -> Response {
        let client = Client::new();
        let response = client.get(API_URL_GET_ACCEPTED_COINS)
            .headers(self.construct_headers(false))
            .send()
            .await.unwrap();

        response
    }

    pub async fn transfer_funds (&self, transfer_data: TransferData) -> Response {
        let payload = TransferData {
            amount: transfer_data.amount,
            recipient: transfer_data.recipient,
            coin: transfer_data.coin,
            blockchain: transfer_data.blockchain,
            api_public_key: match transfer_data.api_public_key {
                Some(api_public_key) => Some(api_public_key),
                None => Some(self.api_public_key)
            },
            api_secret_key: match transfer_data.api_secret_key {
                Some(api_secret_key) => Some(api_secret_key),
                None => Some(self.api_secret_key)
            }
        };
        let client = Client::new();
        let response = client.post(API_URL_TRANSFER_FUNDS)
            .json(&payload)
            .headers(self.construct_headers(true))
            .send()
            .await.unwrap();

        response
    }

    fn construct_headers (&self, secret_key_required: bool) -> HeaderMap {
        match secret_key_required {
            true => {
                let api_secret_key = format!("Bearer {}", self.api_secret_key);
                let mut headers = HeaderMap::new();

                headers.insert("X-API-KEY", HeaderValue::from_str(self.api_public_key.as_str()).unwrap());
                headers.insert("AUTHORIZATION", HeaderValue::from_str(api_secret_key.as_str()).unwrap());
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                headers
            },
            false => {
                let mut headers = HeaderMap::new();

                headers.insert("X-API-KEY", HeaderValue::from_str(self.api_public_key.as_str()).unwrap());
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                headers
            }
        }
    }
}