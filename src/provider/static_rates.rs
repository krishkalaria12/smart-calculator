use std::{collections::HashMap, collections::HashSet, sync::LazyLock};

use crate::provider::error::{ProviderError, Result};

pub static STATIC_FIAT_PER_USD: LazyLock<HashMap<&'static str, f64>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("USD", 1.0);
    map.insert("EUR", 0.92);
    map.insert("GBP", 0.79);
    map.insert("INR", 83.1);
    map.insert("JPY", 150.0);
    map.insert("CNY", 7.2);
    map.insert("AUD", 1.52);
    map.insert("CAD", 1.35);
    map.insert("CHF", 0.88);
    map.insert("KRW", 1330.0);
    map.insert("SGD", 1.34);
    map.insert("HKD", 7.8);
    map.insert("NZD", 1.64);
    map.insert("SEK", 10.5);
    map.insert("NOK", 10.7);
    map.insert("DKK", 6.85);
    map.insert("MXN", 17.1);
    map.insert("BRL", 5.0);
    map.insert("THB", 35.8);
    map.insert("PHP", 56.2);
    map.insert("IDR", 15700.0);
    map.insert("MYR", 4.48);
    map.insert("ZAR", 18.4);
    map.insert("AED", 3.67);
    map.insert("SAR", 3.75);
    map.insert("TRY", 32.1);
    map.insert("PLN", 3.95);
    map.insert("CZK", 23.2);
    map.insert("RUB", 92.0);
    map
});

pub static STATIC_CRYPTO_USD: LazyLock<HashMap<&'static str, f64>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("BTC", 92000.0);
    map.insert("ETH", 3500.0);
    map.insert("SOL", 180.0);
    map.insert("XRP", 2.3);
    map.insert("DOGE", 0.18);
    map.insert("ADA", 0.8);
    map.insert("MATIC", 0.95);
    map.insert("DOT", 7.5);
    map.insert("LTC", 95.0);
    map.insert("AVAX", 42.0);
    map.insert("LINK", 20.0);
    map.insert("UNI", 12.0);
    map.insert("ATOM", 10.0);
    map.insert("XLM", 0.12);
    map.insert("TON", 6.0);
    map.insert("SHIB", 0.000025);
    map.insert("BNB", 650.0);
    map.insert("USDT", 1.0);
    map.insert("USDC", 1.0);
    map
});

// 1. CoinGecko ID map for common tickers
pub static COINGECKO_IDS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("BTC", "bitcoin");
    map.insert("ETH", "ethereum");
    map.insert("SOL", "solana");
    map.insert("XRP", "ripple");
    map.insert("USDT", "tether");
    map.insert("USDC", "usd-coin");
    map.insert("BNB", "binancecoin");
    map.insert("DOGE", "dogecoin");
    map.insert("ADA", "cardano");
    map.insert("MATIC", "matic-network");
    map.insert("DOT", "polkadot");
    map.insert("LTC", "litecoin");
    map.insert("AVAX", "avalanche-2");
    map.insert("LINK", "chainlink");
    map.insert("UNI", "uniswap");
    map.insert("ATOM", "cosmos");
    map.insert("XLM", "stellar");
    map.insert("ALGO", "algorand");
    map.insert("FIL", "filecoin");
    map.insert("NEAR", "near");
    map.insert("APT", "aptos");
    map.insert("ARB", "arbitrum");
    map.insert("OP", "optimism");
    map.insert("SUI", "sui");
    map.insert("SHIB", "shiba-inu");
    map.insert("PEPE", "pepe");
    map.insert("TRX", "tron");
    map.insert("TON", "the-open-network");
    map.insert("HBAR", "hedera-hashgraph");
    map.insert("ICP", "internet-computer");
    map.insert("VET", "vechain");
    map.insert("AAVE", "aave");
    map.insert("MKR", "maker");
    map.insert("CRV", "curve-dao-token");
    map.insert("SNX", "havven");
    map.insert("COMP", "compound-governance-token");
    map.insert("SAND", "the-sandbox");
    map.insert("MANA", "decentraland");
    map.insert("AXS", "axie-infinity");
    map.insert("APE", "apecoin");
    map.insert("FTM", "fantom");
    map.insert("ONE", "harmony");
    map.insert("KAVA", "kava");
    map.insert("ROSE", "oasis-network");
    map.insert("ZEC", "zcash");
    map.insert("XMR", "monero");
    map.insert("DASH", "dash");
    map.insert("ETC", "ethereum-classic");
    map.insert("BCH", "bitcoin-cash");
    map.insert("BSV", "bitcoin-cash-sv");
    map
});

// 2. CoinGecko currency map (fiat/crypto tickers it supports as quote currencies)
pub static COINGECKO_FIATS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut set = HashSet::with_capacity(47); // Pre-allocating capacity makes startup even faster

    let fiats = [
        "usd", "eur", "gbp", "jpy", "aud", "cad", "chf", "cny", "hkd", "nzd", "sek", "krw", "sgd",
        "nok", "mxn", "inr", "rub", "zar", "try", "brl", "twd", "dkk", "pln", "thb", "idr", "huf",
        "czk", "ils", "clp", "php", "aed", "sar", "myr", "ars", "ngn", "pkr", "bdt", "vnd", "egp",
        "uah", "btc", "eth", "bnb", "xrp", "sol", "dot", "ltc",
    ];

    for fiat in fiats {
        set.insert(fiat);
    }

    set
});

pub static STABLECOINS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert("USDT");
    set.insert("USDC");
    set.insert("BUSD");
    set
});

// 2. Binance Quote Assets Set
pub static BINANCE_QUOTES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut set = HashSet::with_capacity(6);
    set.insert("USDT");
    set.insert("USDC");
    set.insert("BTC");
    set.insert("ETH");
    set.insert("BNB");
    set.insert("BUSD");
    set
});

pub static COINCAP_IDS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("BTC", "bitcoin");
    map.insert("ETH", "ethereum");
    map.insert("SOL", "solana");
    map.insert("XRP", "xrp");
    map.insert("USDT", "tether");
    map.insert("USDC", "usd-coin");
    map.insert("BNB", "binance-coin");
    map.insert("DOGE", "dogecoin");
    map.insert("ADA", "cardano");
    map.insert("DOT", "polkadot");
    map.insert("LTC", "litecoin");
    map.insert("AVAX", "avalanche");
    map.insert("LINK", "chainlink");
    map.insert("UNI", "uniswap");
    map.insert("ATOM", "cosmos");
    map.insert("XLM", "stellar");
    map.insert("MATIC", "polygon");
    map.insert("SHIB", "shiba-inu");
    map.insert("TRX", "tron");
    map.insert("TON", "toncoin");
    map.insert("BCH", "bitcoin-cash");
    map.insert("ETC", "ethereum-classic");
    map.insert("XMR", "monero");
    map.insert("ZEC", "zcash");
    map
});

pub fn static_fiat_rate(base: String, target: String) -> Result<f64> {
    let from = STATIC_FIAT_PER_USD.get(base.as_str());
    let to = STATIC_FIAT_PER_USD.get(target.as_str());

    if let Some(from) = from {
        if let Some(to) = to {
            return Ok(to / from);
        }
    }

    Err(ProviderError::FiatRateUnavaliable(base, target))
}

pub fn static_crypto_rate(base: String, target: String) -> Result<f64> {
    let from = STATIC_CRYPTO_USD.get(base.as_str());
    let to = STATIC_CRYPTO_USD.get(target.as_str());

    if let Some(from) = from {
        if let Some(to) = to {
            return Ok(from / to);
        } else {
            let usd_to_target = static_fiat_rate("USD".to_string(), target)?;
            return Ok(from * usd_to_target);
        }
    }

    if let Some(to) = to {
        let from_to_usd = static_fiat_rate(base, "USD".to_string())?;
        return Ok(from_to_usd / to);
    }

    Err(ProviderError::CryptoRateUnavaliable(
        base.clone(),
        target.clone(),
    ))
}
