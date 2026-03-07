use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, EvaluatorError>;

#[derive(Debug, Clone, Error)]
pub enum EvaluatorError {
    #[error("{0} is required")]
    RequiredParameter(String),

    #[error("Currency conversion failed: {0}")]
    CurrencyConversionFailed(String),

    #[error("Crypto conversion failed: {0}")]
    CryptoConversionFailed(String),

    #[error("Date evaluation failed: {0}")]
    DateEvaluationFailed(String),

    #[error("Time evaluation failed: {0}")]
    TimeEvaluationFailed(String),

    #[error("Math evaluation failed: {0}")]
    MathEvaluationFailed(String),

    #[error("Invalid {0}: {0}")]
    Invalid(String, String),
}

impl Serialize for EvaluatorError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
