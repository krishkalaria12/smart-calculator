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
pub trait RateProvider {
    async fn get_fiat_rate(&self, base: &str, target: &str) -> Result<f64, Box<dyn Error>>;

    async fn get_crypto_rate(&self, base: &str, target: &str) -> Result<f64, Box<dyn Error>>;
}

pub struct Config {
    rate_rovider: Option<Arc<dyn RateProvider + Send + Sync>>,
    /** User's local timezone (IANA), defaults to system timezone */
    timezone: Option<String>,
    /** Locale for number formatting, defaults to 'en-US' */
    locale: Option<String>,
    /** Maximum decimal places for results */
    precision: Option<f64>,
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
