pub mod data;
pub mod error;
pub mod evaluators;
pub mod http;
pub mod parser;
pub mod provider;
pub mod types;

use std::sync::Arc;

use crate::{
    error::{Error, Result},
    evaluators::{
        currency::{evaluate_crypto, evaluate_currency},
        date::evaluate_date,
        math::evaluate_math,
    },
    parser::detect_intent,
    provider::DefaultProvider,
    types::{CalculatorResult, Config, Intent},
};

pub async fn calculate(input: &str, options: Option<Config>) -> Result<CalculatorResult> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(Error::RequiredParameter("input".to_string()));
    }

    let options = options.unwrap_or_default();
    let intent = detect_intent(trimmed);

    let provider = options
        .rate_provider()
        .unwrap_or_else(|| Arc::new(DefaultProvider));

    match intent {
        Intent::Currency { amount, from, to } => evaluate_currency(
            amount,
            from,
            to,
            Some(provider.as_ref()),
            options.locale().clone(),
            options.precision(),
        )
        .await
        .map_err(|err| Error::Evaluation(err.to_string())),
        Intent::Crypto { amount, from, to } => evaluate_crypto(
            amount,
            from,
            to,
            Some(provider.as_ref()),
            options.locale().clone(),
            options.precision(),
        )
        .await
        .map_err(|err| Error::Evaluation(err.to_string())),
        Intent::Math { expression } => evaluate_math(
            expression,
            options.locale().clone(),
            options.precision(),
        )
        .map_err(|err| Error::Evaluation(err.to_string())),
        Intent::Date { query } => {
            evaluate_date(query).map_err(|err| Error::Evaluation(err.to_string()))
        }
        other => Err(Error::UnsupportedIntent(format!("{other:?}"))),
    }
}
