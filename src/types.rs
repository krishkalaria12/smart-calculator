use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, error::Error, sync::Arc};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResultType {
    Math,
    Unit,
    Currency,
    Crypto,
    Time,
    Date,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnswerType {
    Number(f64),
    Text(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculatorResult {
    pub res_type: ResultType,
    pub input: String,
    pub result: AnswerType,
    pub formatted: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
}

#[async_trait]
pub trait RateProvider: Send + Sync {
    async fn get_fiat_rate(
        &self,
        base: &str,
        target: &str,
    ) -> std::result::Result<f64, Box<dyn Error>>;

    async fn get_crypto_rate(
        &self,
        base: &str,
        target: &str,
    ) -> std::result::Result<f64, Box<dyn Error>>;
}

#[derive(Clone, Default)]
pub struct Config {
    rate_provider: Option<Arc<dyn RateProvider + Send + Sync>>,
    timezone: Option<String>,
    locale: Option<String>,
    precision: Option<u8>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rate_provider(
        mut self,
        rate_provider: Arc<dyn RateProvider + Send + Sync>,
    ) -> Self {
        self.rate_provider = Some(rate_provider);
        self
    }

    pub fn with_timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = Some(timezone.into());
        self
    }

    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    pub fn with_precision(mut self, precision: u8) -> Self {
        self.precision = Some(precision.clamp(1, 21));
        self
    }

    pub fn rate_provider(&self) -> Option<Arc<dyn RateProvider + Send + Sync>> {
        self.rate_provider.clone()
    }

    pub fn timezone(&self) -> &Option<String> {
        &self.timezone
    }

    pub fn locale(&self) -> &Option<String> {
        &self.locale
    }

    pub fn precision(&self) -> Option<u8> {
        self.precision
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Intent {
    Time {
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        from: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        to: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        time: Option<String>,
    },

    Date {
        query: String,
    },

    Currency {
        amount: f64,
        from: String,
        to: String,
    },

    Crypto {
        amount: f64,
        from: String,
        to: String,
    },

    Unit {
        amount: f64,
        from: String,
        to: String,
        category: String,
    },

    Math {
        expression: String,
    },
}
