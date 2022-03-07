use std::fmt::Display;

/// Errors from the SDK.
#[derive(Debug)]
pub enum Error {
    /// An error from the API.
    Api(String),
    /// A Configuration error.
    Config(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Api(s) => write!(f, "API Error: {}", s),
            Error::Config(s) => write!(f, "Config Error: {}", s),
        }
    }
}

impl std::error::Error for Error {}
