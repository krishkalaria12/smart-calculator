use serde::Serialize;
use thiserror::Error;

use crate::http::error::HttpError;

pub type Result<T> = std::result::Result<T, ProviderError>;

#[derive(Debug, Clone, Error)]
pub enum ProviderError {
    #[error("Static fiat rate unavailable for {0}/{0}")]
    FiatRateUnavaliable(String, String),

    #[error("Static crypto rate unavailable for {0}/{0}")]
    CryptoRateUnavaliable(String, String),

    #[error("{0}")]
    DeserializationError(String),

    #[error("Rate not found")]
    RateNotFound,

    #[error("Unsupported Target: {0}")]
    UnsupportedTarget(String),

    #[error("{0}: pair not found")]
    PairNotFound(String),

    #[error("HTTP request failed: {0}")]
    Http(#[from] HttpError),
}

impl Serialize for ProviderError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
