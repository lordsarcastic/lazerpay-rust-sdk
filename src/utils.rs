use dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Api {
    pub public_key: String,
    pub secret_key: String,
}

#[allow(dead_code)]
pub fn get_api_keys() -> Result<Api, String> {
    dotenv::dotenv().expect("Failed to read .env file");

    match envy::from_env::<Api>() {
        Ok(config) => Ok(config),
        Err(e) => Err(format!("Couldn't read config ({})", e)),
    }
}
