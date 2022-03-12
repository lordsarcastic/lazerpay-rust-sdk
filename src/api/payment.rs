use super::Customer;
use crate::{constants::*, utils::construct_headers, ApiConfig, Error};
use crate::{PaymentConfirmData, PaymentSendData, Response};
use anyhow::Result;
use erased_serde::Serialize;
use reqwest::Client;
use std::collections::HashMap;

/// This is used to create and send payments
pub struct Payment<'l> {
    pub(crate) api_config: &'l ApiConfig,
    pub payment_config: PaymentConfig,
    pub id: Option<String>,
}

/// The necessary details for a payment.
#[derive(Default, Debug)]
pub struct PaymentConfig {
    pub customer: Customer,
    pub reference: String,
    pub coin: String,
    pub amount: String,
    pub currency: String,
    pub accept_partial_payment: bool,
}

impl<'l> Payment<'l> {
    /// Creates a new payment but does not send it.
    pub fn new(api_config: &'l ApiConfig, payment_config: PaymentConfig) -> Self {
        Self {
            api_config,
            payment_config,
            id: None,
        }
    }

    /// Sends the payment request to the server.
    pub async fn send(&mut self) -> Result<Response<PaymentSendData>> {
        let response = Client::new()
            .post(API_URL_INIT_TRANSACTION)
            .json(&self.payment_config.get_payload())
            .headers(construct_headers(self.api_config, true))
            .send()
            .await?;

        let is_success = response.status().is_success();

        let payload: Response<PaymentSendData> = response.json().await?;

        if is_success {
            self.id = Some(payload.data.as_ref().unwrap().address.clone());
        }

        Ok(payload)
    }

    /// Gets the payment status.
    pub async fn check_confirmation(&self) -> Result<Response<PaymentConfirmData>> {
        match &self.id {
            Some(id) => {
                let response = Client::new()
                    .get(format!("{}/{}", API_URL_CONFIRM_TRANSACTION, id))
                    .headers(construct_headers(self.api_config, false))
                    .send()
                    .await?;

                Ok(response.json().await?)
            }
            None => Err(Error::Api("Payment not sent or unsuccessfully processed".into()).into()),
        }
    }
}

impl PaymentConfig {
    /// Generates request body for payment request.
    fn get_payload(&self) -> HashMap<&'static str, Box<dyn Serialize>> {
        let mut payload: HashMap<_, Box<dyn Serialize + 'static>> = HashMap::new();
        payload.insert("reference", Box::new(self.reference.clone()));
        payload.insert("amount", Box::new(self.amount.clone()));
        payload.insert("customer_name", Box::new(self.customer.name.clone()));
        payload.insert("customer_email", Box::new(self.customer.email.clone()));
        payload.insert("coin", Box::new(self.coin.clone()));
        payload.insert("currency", Box::new(self.currency.clone()));
        payload.insert(
            "accept_partial_payment",
            Box::new(self.accept_partial_payment),
        );
        payload
    }
}
