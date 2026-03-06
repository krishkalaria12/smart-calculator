use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub(crate) struct ExchangeResponse {
    pub rates: HashMap<String, f64>,
}

#[derive(Deserialize)]
pub(crate) struct BinancePriceResponse {
    pub price: String,
}

#[derive(Deserialize)]
pub(crate) struct CoinCapResponse {
    pub data: CoinCapData,
}

#[derive(Deserialize)]
pub(crate) struct CoinCapData {
    #[serde(rename = "priceUsd")]
    pub price_usd: String,
}
