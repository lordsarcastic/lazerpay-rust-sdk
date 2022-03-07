use super::{Payment, PaymentConfig, Transfer, TransferConfig};
use crate::{
    constants::{API_URL_GET_ACCEPTED_COINS, API_URL_GET_RATE},
    utils::{self, construct_headers},
    AcceptedCoinData, RateGetResponse, Response,
};
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

/// The LazerPay API client.
pub struct Lazerpay(ApiConfig);

/// The API configuration.
#[derive(Deserialize, Debug)]
pub struct ApiConfig {
    #[serde(rename = "lazerpay_public_key")]
    pub public_key: String,
    #[serde(rename = "lazerpay_secret_key")]
    pub secret_key: String,
}

impl Lazerpay {
    /// Creates a new Lazerpay API client.
    pub fn new() -> Result<Self> {
        Ok(Lazerpay(utils::get_api_keys()?))
    }

    /// Initializes a new payment.
    pub fn create_payment(&self, payment_config: PaymentConfig) -> Payment {
        Payment::new(&self.0, payment_config)
    }

    /// Initializes a new transfer.
    pub fn create_transfer(&self, transfer_config: TransferConfig) -> Transfer {
        Transfer::new(&self.0, transfer_config)
    }

    /// Fetches information about coins supported by Lazerpay.
    pub async fn get_accepted_coins(&self) -> Result<Response<Vec<AcceptedCoinData>>> {
        let response = Client::new()
            .get(API_URL_GET_ACCEPTED_COINS)
            .headers(construct_headers(&self.0, false))
            .send()
            .await?;

        Ok(response.json().await?)
    }

    /// Fetches the latest rate for a given coin and currency.
    pub async fn get_rate<S: Into<String>>(&self, currency: S, coin: S) -> Result<RateGetResponse> {
        let response = Client::new()
            .get(format!(
                "{}?currency={}&coin={}",
                API_URL_GET_RATE,
                currency.into(),
                coin.into()
            ))
            .headers(construct_headers(&self.0, false))
            .send()
            .await?;

        Ok(response.json().await?)
    }
}
