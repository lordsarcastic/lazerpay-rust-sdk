use crate::{
    constants::API_URL_TRANSFER_FUNDS, utils::construct_headers, ApiConfig, Response,
    TransferSendData,
};
use anyhow::Result;
use erased_serde::Serialize;
use reqwest::Client;
use std::collections::HashMap;

/// This is used to create and send transfers.
pub struct Transfer<'l> {
    pub(crate) api_config: &'l ApiConfig,
    pub transfer_config: TransferConfig,
    pub id: Option<String>,
}

/// The necessary details for a transfer.
pub struct TransferConfig {
    pub amount: i128,
    pub recipient: String,
    pub coin: String,
    pub blockchain: String,
}

impl<'l> Transfer<'l> {
    /// Creates a new transfer but does not send it.
    pub fn new(api_config: &'l ApiConfig, transfer_config: TransferConfig) -> Self {
        Self {
            api_config,
            transfer_config,
            id: None,
        }
    }

    /// Sends the transfer request to the server.
    pub async fn send(&self) -> Result<Response<TransferSendData>> {
        let response = Client::new()
            .post(API_URL_TRANSFER_FUNDS)
            .json(&self.transfer_config.get_payload())
            .headers(construct_headers(self.api_config, true))
            .send()
            .await?;

        Ok(response.json().await?)
    }
}

impl TransferConfig {
    /// Generates the request body for the transfer request.
    fn get_payload(&self) -> HashMap<&'static str, Box<dyn Serialize>> {
        let mut payload: HashMap<_, Box<dyn Serialize>> = HashMap::new();
        payload.insert("amount", Box::new(self.amount.clone()));
        payload.insert("recipient", Box::new(self.recipient.clone()));
        payload.insert("coin", Box::new(self.coin.clone()));
        payload.insert("blockchain", Box::new(self.blockchain.clone()));
        payload
    }
}
