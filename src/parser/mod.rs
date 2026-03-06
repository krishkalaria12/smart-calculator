use regex::Regex;
use std::sync::LazyLock;

use crate::{
    data::currencies::{resolve_crypto, resolve_fiat},
    types::Intent,
};

pub struct AppRegexes {
    pub whitespace: Regex,
    pub conversion_pattern: Regex,
    pub prefixes: Regex,
    pub currency_symbol_prefix: Regex,
}

pub static REGEXES: LazyLock<AppRegexes> = LazyLock::new(|| {
    AppRegexes {
        whitespace: Regex::new(r"\s+").unwrap(),

        conversion_pattern: Regex::new(
            r"(?i)^([\d.,]+)?\s*([a-zA-Z$€£¥₹₩₽₺₦₵₪฿]+(?:\s+[a-zA-Z]+)?)\s+(?:to|in|into|as|=)\s+([a-zA-Z$€£¥₹₩₽₺₦₵₪฿]+(?:\s+[a-zA-Z]+)?)$"
        ).unwrap(),

        prefixes: Regex::new(r"(?i)^(?:convert|how much is|what(?:'s|s| is))\s+").unwrap(),

        currency_symbol_prefix: Regex::new(r"^([$€£¥₹₩₽₺₦₵₪฿])\s*([\d.,]+)").unwrap(),
    }
});

pub fn detect_intent(input: String) -> Intent {
    let trimmed = normalize_whitespace(input);
    if trimmed.is_empty() {
        return Intent::Math {
            expression: '0'.to_string(),
        };
    }

    if let Some(intent) = try_currency_or_crypto_intent(trimmed) {
        return intent;
    }

    todo!()
}

fn try_currency_or_crypto_intent(input: String) -> Option<Intent> {
    let normalised = normalize_conversion_input(input);
    let Some(caps) = REGEXES.conversion_pattern.captures(&normalised) else {
        return None;
    };

    let amount: f64 = caps
        .get(1)
        .map(|m| m.as_str().replace(',', "").parse::<f64>().unwrap_or(1.0))
        .unwrap_or(1.0);
    let from_token = caps.get(2).unwrap().as_str().trim().to_string();
    let to_token = caps.get(3).unwrap().as_str().trim().to_string();

    let from_crypto = resolve_crypto(from_token.clone());
    let to_crypto = resolve_crypto(to_token.clone());
    let from_fiat = resolve_fiat(from_token);
    let to_fiat = resolve_fiat(to_token);

    if let Some(from) = from_crypto.clone() {
        if let Some(to) = to_crypto.clone().or(to_fiat.clone()) {
            // Crypto -> Crypto
            return Some(Intent::Crypto { amount, from, to });
        }
    }

    if let Some(from) = from_fiat {
        if let Some(to) = to_crypto {
            // Fiat -> Crypto
            return Some(Intent::Crypto { amount, from, to });
        } else if let Some(to) = to_fiat {
            // Fiat -> Fiat
            return Some(Intent::Currency { amount, from, to });
        }
    }

    None
}

fn normalize_whitespace(input: String) -> String {
    let trimmed = input.trim();
    REGEXES.whitespace.replace_all(trimmed, " ").into_owned()
}

pub fn normalize_conversion_input(input: String) -> String {
    let trimmed = input.trim();
    let mut normalized = REGEXES.whitespace.replace_all(trimmed, " ").into_owned();

    normalized = REGEXES.prefixes.replace(&normalized, "").into_owned();

    normalized = REGEXES
        .currency_symbol_prefix
        .replace(&normalized, "$2 $1")
        .into_owned();

    normalized
}
