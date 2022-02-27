use erased_serde::Serialize;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json;
use serde_json::{from_str, Value};
use std::collections::HashMap;

use crate::constants::*;

pub struct Payment {
    pub api_public_key: String,
    pub api_secret_key: String,
}

impl Payment {
    pub fn new(api_public_key: String, api_secret_key: String) -> Self {
        Payment {
            api_public_key: api_public_key,
            api_secret_key: api_secret_key,
        }
    }

    pub async fn initialize_payment(
        &self,
        reference: String,
        amount: String,
        customer_name: String,
        customer_email: String,
        coin: String,
        currency: String,
        api_public_key: String,
        accept_partial_payment: bool,
    ) -> Value {
        let mut payload: HashMap<&str, Box<dyn Serialize + 'static>> = HashMap::new();
        payload.insert("reference", Box::new(reference));
        payload.insert("amount", Box::new(amount));
        payload.insert("customer_name", Box::new(customer_name));
        payload.insert("customer_email", Box::new(customer_email));
        payload.insert("coin", Box::new(coin));
        payload.insert("currency", Box::new(currency));
        payload.insert("api_public_key", Box::new(api_public_key));
        payload.insert("accept_partial_payment", Box::new(accept_partial_payment));

        let client = Client::new();
        let response = client
            .post(API_URL_INIT_TRANSACTION)
            .json(&payload)
            .headers(self.construct_headers(true))
            .send()
            .await
            .unwrap();

        self.convert_string_to_json(response.text().await.unwrap())
    }

    pub async fn confirm_payment(&self, identifier: String) -> Value {
        let client = Client::new();
        let response = client
            .get(format!("{}/{}", API_URL_CONFIRM_TRANSACTION, identifier))
            .headers(self.construct_headers(false))
            .send()
            .await
            .unwrap();

        self.convert_string_to_json(response.text().await.unwrap())
    }

    pub async fn get_accepted_coins(&self) -> Value {
        let client = Client::new();
        let response = client
            .get(API_URL_GET_ACCEPTED_COINS)
            .headers(self.construct_headers(false))
            .send()
            .await
            .unwrap();

        self.convert_string_to_json(response.text().await.unwrap())
    }

    pub async fn get_rate(&self, currency: String, coin: String) -> Value {
        let client = Client::new();
        let response = client
            .get(format!(
                "{}?currency={}&coin={}",
                API_URL_GET_RATE, currency, coin
            ))
            .headers(self.construct_headers(false))
            .send()
            .await
            .unwrap();

        self.convert_string_to_json(response.text().await.unwrap())
    }

    pub async fn transfer_funds(
        &self,
        amount: i32,
        recipient: String,
        coin: String,
        blockchain: String,
        api_public_key: String,
        api_secret_key: String,
    ) -> Value {
        let mut payload: HashMap<&str, Box<dyn Serialize + 'static>> = HashMap::new();
        payload.insert("amount", Box::new(amount));
        payload.insert("recipient", Box::new(recipient));
        payload.insert("coin", Box::new(coin));
        payload.insert("blockchain", Box::new(blockchain));
        payload.insert("api_public_key", Box::new(api_public_key));
        payload.insert("api_secret_key", Box::new(api_secret_key));

        let client = Client::new();
        let response = client
            .post(API_URL_TRANSFER_FUNDS)
            .json(&payload)
            .headers(self.construct_headers(true))
            .send()
            .await
            .unwrap();

        self.convert_string_to_json(response.text().await.unwrap())
    }

    fn construct_headers(&self, secret_key_required: bool) -> HeaderMap {
        match secret_key_required {
            true => {
                let api_secret_key = format!("Bearer {}", self.api_secret_key);
                let mut headers = HeaderMap::new();

                headers.insert(
                    "X-API-KEY",
                    HeaderValue::from_str(self.api_public_key.as_str()).unwrap(),
                );
                headers.insert(
                    "AUTHORIZATION",
                    HeaderValue::from_str(api_secret_key.as_str()).unwrap(),
                );
                headers
            }
            false => {
                let mut headers = HeaderMap::new();

                headers.insert(
                    "X-API-KEY",
                    HeaderValue::from_str(self.api_public_key.as_str()).unwrap(),
                );
                headers
            }
        }
    }

    fn convert_string_to_json(&self, response_body: String) -> Value {
        from_str::<Value>(&response_body).unwrap()
    }
}
