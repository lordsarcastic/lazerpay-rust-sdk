use std;

use crate::constants;

pub fn validate_env_presence() -> Result<(), String> {
    if let Ok(_api_key) = std::env::var(constants::API_KEY_ENV_VAR) {
        ()
    }
    Err(format!("'{}' environment variable is not set", constants::API_KEY_ENV_VAR))
}