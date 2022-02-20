mod payment;
mod apis;

pub struct Lazerpay {
    pub api_public_key: String,
    pub api_secret_key: String,
    pub payment: payment::Payment
}

impl Lazerpay {
    pub fn new (api_public_key: String, api_secret_key: String) -> Self {
        Lazerpay {
            api_public_key: api_public_key.clone(),
            api_secret_key: api_secret_key.clone(),
            payment: payment::Payment::new(api_public_key, api_secret_key),
        }
    }
}