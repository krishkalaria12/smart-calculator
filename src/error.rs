use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("{0} is required")]
    RequiredParameter(String),

    #[error("Unsupported intent: {0}")]
    UnsupportedIntent(String),

    #[error("Evaluation failed: {0}")]
    Evaluation(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
