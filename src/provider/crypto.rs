use std::{collections::HashMap, error::Error};

use crate::{
    http::fetch_json,
    provider::{
        error::{ProviderError, Result},
        models::{BinancePriceResponse, CoinCapResponse},
        static_rates::{
            COINCAP_IDS, COINGECKO_FIATS, COINGECKO_IDS, static_crypto_rate as static_crypto,
        },
    },
};

type NestedRates = HashMap<String, HashMap<String, f64>>;

async fn coingecko_rate(from: &str, to: &str) -> Result<f64> {
    let from_id = COINGECKO_IDS.get(from);
    let to_id = COINGECKO_IDS.get(to);

    if let Some(from_id) = from_id {
        let vs_currency = if let Some(to_id) = to_id {
            to_id.to_lowercase()
        } else {
            to.to_lowercase()
        };

        if COINGECKO_FIATS.contains(vs_currency.as_str()) && to_id.is_none() {
            return Err(ProviderError::UnsupportedTarget(to.to_string()));
        }

        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={from_id}&vs_currencies={vs_currency}"
        );
        let data = fetch_json(&url, None).await?;

        let parsed: NestedRates = serde_json::from_value(data)
            .map_err(|e| ProviderError::DeserializationError(e.to_string()))?;

        if let Some(rate) = parsed.get(*from_id) {
            return rate
                .get(vs_currency.as_str())
                .copied()
                .ok_or(ProviderError::RateNotFound);
        }
    }

    if let Some(to_id) = to_id {
        let fiat = from.to_lowercase();
        if COINGECKO_FIATS.contains(fiat.as_str()) {
            return Err(ProviderError::UnsupportedTarget(to.to_string()));
        }

        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={to_id}&vs_currencies={fiat}"
        );
        let data = fetch_json(&url, None).await?;

        let parsed: NestedRates = serde_json::from_value(data)
            .map_err(|e| ProviderError::DeserializationError(e.to_string()))?;

        if let Some(rate) = parsed.get(*to_id) {
            let final_rate = rate
                .get(fiat.as_str())
                .copied()
                .ok_or(ProviderError::RateNotFound)?;
            return Ok(1_f64 / final_rate);
        }
    }

    Err(ProviderError::PairNotFound("coingecko".to_string()))
}

async fn binance_direct(from: &str, to: &str) -> Option<f64> {
    let symbol = format!("{from}{to}");
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={symbol}");

    let data = fetch_json(&url, None).await.ok()?;
    let parsed: BinancePriceResponse = serde_json::from_value(data).ok()?;
    parsed.price.parse::<f64>().ok()
}

async fn binance_rate(from: &str, to: &str) -> Result<f64> {
    if let Some(rate) = binance_direct(from, to).await {
        return Ok(rate);
    }

    if let Some(rate) = binance_direct(to, from).await {
        return Ok(1_f64 / rate);
    }

    if from != "USDT" && to != "USDT" {
        let from_usdt = binance_direct(from, "USDT").await;
        let to_usdt = binance_direct(to, "USDT").await;

        if let (Some(from_usdt), Some(to_usdt)) = (from_usdt, to_usdt) {
            return Ok(from_usdt / to_usdt);
        }
    }

    Err(ProviderError::PairNotFound("binance".to_string()))
}

async fn get_coincap_usd_price(ticker: &str) -> Result<f64> {
    if ticker == "USD" {
        return Ok(1.0);
    }

    let id = COINCAP_IDS.get(ticker).ok_or_else(|| {
        ProviderError::CryptoRateUnavaliable(ticker.to_string(), "USD".to_string())
    })?;

    let url = format!("https://api.coincap.io/v2/assets/{id}");
    let json = fetch_json(&url, None).await?;

    let response: CoinCapResponse = serde_json::from_value(json)
        .map_err(|e| ProviderError::DeserializationError(e.to_string()))?;

    response
        .data
        .price_usd
        .parse::<f64>()
        .map_err(|_| ProviderError::CryptoRateUnavaliable(ticker.to_string(), "USD".to_string()))
}

pub async fn coincap_rate(from: &str, to: &str) -> Result<f64> {
    let is_fiat = |t: &str| !COINCAP_IDS.contains_key(t) && t != "USD";

    if to == "USD" {
        return get_coincap_usd_price(from).await;
    }

    if is_fiat(to) {
        return Err(ProviderError::FiatRateUnavaliable(
            from.to_string(),
            to.to_string(),
        ));
    }

    if from == "USD" {
        let price = get_coincap_usd_price(to).await?;
        return if price == 0.0 {
            Ok(0.0)
        } else {
            Ok(1.0 / price)
        };
    }

    let from_usd = get_coincap_usd_price(from).await?;
    let to_usd = get_coincap_usd_price(to).await?;

    if to_usd == 0.0 {
        return Err(ProviderError::CryptoRateUnavaliable(
            from.to_string(),
            to.to_string(),
        ));
    }

    Ok(from_usd / to_usd)
}

fn is_crypto(ticker: &str) -> bool {
    COINGECKO_IDS.contains_key(ticker)
}

async fn try_crypto_to_crypto(from: &str, to: &str) -> Option<f64> {
    if let Ok(rate) = coingecko_rate(from, to).await {
        return Some(rate);
    }
    if let Ok(rate) = binance_rate(from, to).await {
        return Some(rate);
    }
    if let Ok(rate) = coincap_rate(from, to).await {
        return Some(rate);
    }
    static_crypto(from.to_string(), to.to_string()).ok()
}

async fn try_crypto_to_usd(from: &str) -> Option<f64> {
    if let Ok(rate) = coingecko_rate(from, "USD").await {
        return Some(rate);
    }
    if let Ok(rate) = binance_rate(from, "USDT").await {
        return Some(rate);
    }
    if let Ok(rate) = coincap_rate(from, "USD").await {
        return Some(rate);
    }
    static_crypto(from.to_string(), "USD".to_string()).ok()
}

async fn try_usd_to_crypto(to: &str) -> Option<f64> {
    if let Ok(rate) = coingecko_rate("USD", to).await {
        if rate > 0.0 {
            return Some(rate);
        }
    }
    if let Ok(price) = coingecko_rate(to, "USD").await {
        if price > 0.0 {
            return Some(1.0 / price);
        }
    }
    if let Ok(price) = coincap_rate(to, "USD").await {
        if price > 0.0 {
            return Some(1.0 / price);
        }
    }
    static_crypto("USD".to_string(), to.to_string()).ok()
}

pub async fn get_crypto_rate_with_fiat_fallback<F, Fut>(
    from: &str,
    to: &str,
    get_fiat: F,
) -> Result<f64>
where
    F: Fn(String, String) -> Fut,
    Fut: std::future::Future<Output = std::result::Result<f64, Box<dyn Error>>>,
{
    let from_is_crypto = is_crypto(from);
    let to_is_crypto = is_crypto(to);
    let to_is_fiat = !to_is_crypto;
    let from_is_fiat = !from_is_crypto;

    if let Some(rate) = try_crypto_to_crypto(from, to).await {
        return Ok(rate);
    }

    if from_is_crypto && to_is_fiat {
        let crypto_to_usd = try_crypto_to_usd(from).await.ok_or_else(|| {
            ProviderError::CryptoRateUnavaliable(from.to_string(), "USD".to_string())
        })?;

        if to == "USD" || to == "USDT" || to == "USDC" {
            return Ok(crypto_to_usd);
        }

        let usd_to_fiat = get_fiat("USD".to_string(), to.to_string())
            .await
            .map_err(|e| ProviderError::DeserializationError(e.to_string()))?;
        return Ok(crypto_to_usd * usd_to_fiat);
    }

    if from_is_fiat && to_is_crypto {
        let fiat_to_usd = if from == "USD" {
            Ok(1.0)
        } else {
            get_fiat(from.to_string(), "USD".to_string())
                .await
                .map_err(|e| ProviderError::DeserializationError(e.to_string()))
        }?;

        let usd_to_crypto = try_usd_to_crypto(to).await.ok_or_else(|| {
            ProviderError::CryptoRateUnavaliable("USD".to_string(), to.to_string())
        })?;

        return Ok(fiat_to_usd * usd_to_crypto);
    }

    static_crypto(from.to_string(), to.to_string())
}
