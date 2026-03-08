use fixed_decimal::{Decimal as FixedDecimal, FloatPrecision};
use iso_currency::Currency;
use serde_json::json;
use std::collections::HashMap;

use crate::data::currencies::CRYPTO_CURRENCIES;
use crate::{
    data::currencies::FIAT_CURRENCIES,
    evaluators::error::{EvaluatorError, Result},
    formatting::{MAX_DISPLAY_FRACTION_DIGITS, format_display_number, normalize_locale},
    types::{AnswerType, CalculatorResult, RateProvider, ResultType},
};

pub async fn evaluate_currency(
    amount: f64,
    from: String,
    to: String,
    provider: Option<&dyn RateProvider>,
    locale: Option<String>,
    precision: Option<u8>,
) -> Result<CalculatorResult> {
    let from = normalize_code(from);
    let to = normalize_code(to);
    let input_str = format!("{amount} {from} to {to}");

    let provider = require_provider(provider)?;
    let precision = normalize_precision(precision, 6);
    let locale = normalize_locale(locale);

    let rate = provider
        .get_fiat_rate(&from, &to)
        .await
        .map_err(|err| EvaluatorError::CurrencyConversionFailed(err.to_string()))?;

    let converted = amount * rate;
    let rounded = round_to_significant_digits(converted, precision);
    let formatted = format_currency_value(rounded, &to, &locale, precision);

    let from_name = lookup_fiat_name(&from);
    let to_name = lookup_fiat_name(&to);
    let metadata = build_metadata(&from, &from_name, &to, &to_name, rate, amount);

    Ok(CalculatorResult {
        res_type: ResultType::Currency,
        input: input_str,
        result: AnswerType::Number(rounded),
        formatted,
        metadata: Some(metadata),
    })
}

pub async fn evaluate_crypto(
    amount: f64,
    from: String,
    to: String,
    provider: Option<&dyn RateProvider>,
    locale: Option<String>,
    precision: Option<u8>,
) -> Result<CalculatorResult> {
    let from = normalize_code(from);
    let to = normalize_code(to);
    let input_str = format!("{amount} {from} to {to}");

    let provider = require_provider(provider)?;
    let precision = normalize_precision(precision, 10);
    let locale = normalize_locale(locale);

    let rate = provider
        .get_crypto_rate(&from, &to)
        .await
        .map_err(|err| EvaluatorError::CryptoConversionFailed(err.to_string()))?;

    let converted = amount * rate;
    let rounded = round_to_significant_digits(converted, precision);

    let to_is_fiat = FIAT_CURRENCIES.pin().contains_key(to.as_str());
    let formatted = if to_is_fiat {
        format_currency_value(rounded, &to, &locale, precision)
    } else {
        format_asset_value(rounded, &to, &locale, precision)
    };

    let from_name = lookup_asset_name(&from);
    let to_name = lookup_asset_name(&to);
    let metadata = build_metadata(&from, &from_name, &to, &to_name, rate, amount);

    Ok(CalculatorResult {
        res_type: ResultType::Crypto,
        input: input_str,
        result: AnswerType::Number(rounded),
        formatted,
        metadata: Some(metadata),
    })
}

fn normalize_precision(precision: Option<u8>, default: u8) -> u8 {
    precision.unwrap_or(default).clamp(1, 21)
}

fn normalize_code(code: String) -> String {
    code.trim().to_uppercase()
}

fn require_provider(provider: Option<&dyn RateProvider>) -> Result<&dyn RateProvider> {
    provider.ok_or_else(|| EvaluatorError::RequiredParameter("provider".to_string()))
}

fn round_to_significant_digits(value: f64, precision: u8) -> f64 {
    if !value.is_finite() || value == 0.0 {
        return value;
    }

    FixedDecimal::try_from_f64(value, FloatPrecision::SignificantDigits(precision))
        .ok()
        .and_then(|decimal| decimal.to_string().parse::<f64>().ok())
        .unwrap_or(value)
}

fn format_asset_value(value: f64, code: &str, locale_str: &str, precision: u8) -> String {
    let numeric = format_display_number(
        value,
        locale_str,
        precision.min(MAX_DISPLAY_FRACTION_DIGITS),
    );
    format!("{numeric} {code}")
}

fn format_currency_value(value: f64, code: &str, locale_str: &str, precision: u8) -> String {
    let numeric = format_display_number(
        value,
        locale_str,
        precision.min(MAX_DISPLAY_FRACTION_DIGITS),
    );

    match Currency::from_code(code) {
        Some(currency) => {
            let symbol = currency.symbol().to_string();
            if symbol == "¤" {
                format!("{numeric} {code}")
            } else {
                format!("{symbol}{numeric}")
            }
        }
        None => format!("{numeric} {code}"),
    }
}

fn lookup_fiat_name(code: &str) -> String {
    FIAT_CURRENCIES
        .pin()
        .get(code)
        .map(|name| (*name).to_string())
        .unwrap_or_else(|| code.to_string())
}

fn lookup_asset_name(code: &str) -> String {
    if let Some(name) = FIAT_CURRENCIES.pin().get(code) {
        return (*name).to_string();
    }
    if let Some(name) = CRYPTO_CURRENCIES.pin().get(code) {
        return (*name).to_string();
    }
    code.to_string()
}

fn build_metadata(
    from_code: &str,
    from_name: &str,
    to_code: &str,
    to_name: &str,
    rate: f64,
    amount: f64,
) -> HashMap<String, serde_json::Value> {
    let mut metadata = HashMap::new();
    metadata.insert(
        "from".to_string(),
        json!({ "code": from_code, "name": from_name }),
    );
    metadata.insert(
        "to".to_string(),
        json!({ "code": to_code, "name": to_name }),
    );
    metadata.insert("rate".to_string(), json!(rate));
    metadata.insert("amount".to_string(), json!(amount));
    metadata
}
