use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, HttpError>;

#[derive(Debug, Clone, Error)]
pub enum HttpError {
    #[error("HTTP request failed: {0}")]
    HttpRequestError(String),

    #[error("HTTP request failed with status {status} ({status_text}) for {url}")]
    HttpResponseStatusError {
        url: String,
        status: u16,
        status_text: String,
    },

    #[error("HTTP response decode failed: {0}")]
    HttpResponseDecodeError(String),

    #[error("Request timeout: {0}")]
    RequestTimeoutError(String),
}

impl Serialize for HttpError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
