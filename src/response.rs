use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub message: String,
    pub status: Option<String>,
    pub data: Option<T>,
    pub error: Option<String>,
    pub status_code: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateGetResponse {
    pub message: String,
    pub status: Option<String>,
    pub rate: f64,
    pub error: Option<String>,
    pub status_code: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSendData {
    pub reference: String,
    pub business_name: String,
    pub business_email: String,
    pub business_logo: String,
    pub customer_name: String,
    pub customer_email: String,
    pub address: String,
    pub crypto_amount: f64,
    pub fiat_amount: f64,
    pub fee_in_crypto: f64,
    pub currency: String,
    pub coin: String,
    pub accept_partial_payment: bool,
    pub network: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentConfirmData {
    pub id: String,
    pub reference: String,
    pub sender_address: Option<String>,
    pub recipient_address: String,
    pub actual_amount: f64,
    pub amount_paid: Option<String>,
    pub amount_paid_fiat: Option<String>,
    pub fiat_amount: f64,
    pub amount_recieved: Option<String>,
    pub amount_recieved_fiat: Option<String>,
    pub coin: String,
    pub currency: String,
    pub hash: Option<String>,
    pub block_number: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub accept_partial_payment: bool,
    pub status: String,
    pub network: String,
    pub blockchain: String,
    pub fee_in_crypto: f64,
    pub customer: PaymentConfirmCustomerData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentConfirmCustomerData {
    pub customer_name: String,
    pub customer_email: String,
    pub customer_phone: Option<String>,
    pub id: String,
    pub network: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferSendData {
    pub id: String,
    pub transaction_hash: String,
    pub wallet_address: String,
    pub amount: f64,
    pub coin: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptedCoinData {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub logo: String,
    pub address: String,
    pub network: String,
    pub blockchain: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}
