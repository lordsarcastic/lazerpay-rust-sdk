use crate::{ApiConfig, Error};
use anyhow::Result;
use dotenv;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::header::{HeaderMap, HeaderValue};

/// Generates random reference for payment config.
pub fn generate_unique_reference() -> String {
    let random_str: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    format!("reference #{}", random_str)
}

pub(crate) fn get_api_keys() -> Result<ApiConfig> {
    dotenv::dotenv().expect("Failed to read .env file");

    match envy::from_env::<ApiConfig>() {
        Ok(config) => Ok(config),
        Err(e) => Err(Error::Config(format!("Couldn't read config ({})", e)).into()),
    }
}

pub(crate) fn construct_headers(api_config: &ApiConfig, secret_key_required: bool) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-API-KEY",
        HeaderValue::from_str(&api_config.public_key).unwrap(),
    );

    if secret_key_required {
        headers.insert(
            "AUTHORIZATION",
            HeaderValue::from_str(&format!("Bearer {}", api_config.secret_key)).unwrap(),
        );
    }

    headers
}
