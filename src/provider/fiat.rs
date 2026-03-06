use crate::{
    http::fetch_json,
    provider::{
        error::{ProviderError, Result},
        models::ExchangeResponse,
        static_rates::static_fiat_rate,
    },
};

type NestedRates = std::collections::HashMap<String, std::collections::HashMap<String, f64>>;

async fn frankfurter_rate(base: &str, target: &str) -> Result<f64> {
    let url = format!("https://api.frankfurter.dev/v1/latest?base={base}&symbols={target}");
    let data = fetch_json(&url, None).await?;

    let parsed: ExchangeResponse = serde_json::from_value(data)
        .map_err(|e| ProviderError::DeserializationError(e.to_string()))?;

    parsed
        .rates
        .get(target)
        .copied()
        .ok_or(ProviderError::RateNotFound)
}

async fn exchange_rate_api_rate(base: &str, target: &str) -> Result<f64> {
    let url = format!("https://open.er-api.com/v6/latest/{base}");
    let data = fetch_json(&url, None).await?;

    let parsed: ExchangeResponse = serde_json::from_value(data)
        .map_err(|e| ProviderError::DeserializationError(e.to_string()))?;

    parsed
        .rates
        .get(target)
        .copied()
        .ok_or(ProviderError::RateNotFound)
}

async fn fawaz_currency_rate(base: &str, target: &str) -> Result<f64> {
    let base = base.to_lowercase();
    let target = target.to_lowercase();

    let url = format!(
        "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1/currencies/{base}.json"
    );
    let data = fetch_json(&url, None).await?;

    let parsed: NestedRates = serde_json::from_value(data)
        .map_err(|e| ProviderError::DeserializationError(e.to_string()))?;

    parsed
        .get(base.as_str())
        .and_then(|rates| rates.get(target.as_str()))
        .copied()
        .ok_or(ProviderError::RateNotFound)
}

pub async fn get_fiat_rate(base: &str, target: &str) -> Result<f64> {
    if let Ok(rate) = frankfurter_rate(base, target).await {
        return Ok(rate);
    }

    if let Ok(rate) = exchange_rate_api_rate(base, target).await {
        return Ok(rate);
    }

    if let Ok(rate) = fawaz_currency_rate(base, target).await {
        return Ok(rate);
    }

    static_fiat_rate(base.to_string(), target.to_string())
}
