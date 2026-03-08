use fixed_decimal::{Decimal as FixedDecimal, FloatPrecision};
use serde_json::json;
use std::collections::HashMap;

use crate::data::units::{convert_unit, lookup_unit};
use crate::evaluators::error::{EvaluatorError, Result};
use crate::formatting::{MAX_DISPLAY_FRACTION_DIGITS, format_display_number, normalize_locale};
use crate::types::{AnswerType, CalculatorResult, ResultType};

const DEFAULT_PRECISION: u8 = 10;

pub fn evaluate_unit(
    amount: f64,
    from_token: String,
    to_token: String,
    locale: Option<String>,
    precision: Option<u8>,
) -> Result<CalculatorResult> {
    let input_str = format!("{amount} {from_token} to {to_token}");
    let from = lookup_unit(&from_token).ok_or_else(|| {
        EvaluatorError::UnitConversionFailed(format!("Unknown unit: '{from_token}'"))
    })?;
    let to = lookup_unit(&to_token).ok_or_else(|| {
        EvaluatorError::UnitConversionFailed(format!("Unknown unit: '{to_token}'"))
    })?;

    if from.category_name != to.category_name {
        return Err(EvaluatorError::UnitConversionFailed(format!(
            "Cannot convert {} to {}",
            from.category_name, to.category_name
        )));
    }

    let precision = precision.unwrap_or(DEFAULT_PRECISION).clamp(1, 21);
    let locale = normalize_locale(locale);
    let converted =
        convert_unit(amount, &from, &to).map_err(EvaluatorError::UnitConversionFailed)?;
    let rounded = round_to_significant_digits(converted, precision);
    let numeric =
        format_display_number(rounded, &locale, precision.min(MAX_DISPLAY_FRACTION_DIGITS));
    let formatted = format!("{numeric} {}", to.def.name);

    let mut metadata = HashMap::new();
    metadata.insert("fromUnit".to_string(), json!(from.def.name));
    metadata.insert("toUnit".to_string(), json!(to.def.name));
    metadata.insert("category".to_string(), json!(from.category_name));
    metadata.insert("amount".to_string(), json!(amount));

    Ok(CalculatorResult {
        res_type: ResultType::Unit,
        input: input_str,
        result: AnswerType::Number(rounded),
        formatted,
        metadata: Some(metadata),
    })
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
