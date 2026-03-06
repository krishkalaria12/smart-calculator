use crate::{
    data::currencies::{resolve_crypto, resolve_fiat},
    types::Intent,
};

use super::normalize::{REGEXES, normalize_conversion_input, normalize_whitespace};

pub fn detect_intent(input: &str) -> Intent {
    let trimmed = normalize_whitespace(input);
    if trimmed.is_empty() {
        return Intent::Math {
            expression: "0".to_string(),
        };
    }

    if let Some(intent) = try_currency_or_crypto_intent(&trimmed) {
        return intent;
    }

    Intent::Math {
        expression: trimmed,
    }
}

fn try_currency_or_crypto_intent(input: &str) -> Option<Intent> {
    let normalized = normalize_conversion_input(input);
    let caps = REGEXES.conversion_pattern.captures(&normalized)?;

    let amount = caps
        .get(1)
        .map(|m| m.as_str().replace(',', "").parse::<f64>().unwrap_or(1.0))
        .unwrap_or(1.0);

    let from_token = caps.get(2)?.as_str().trim();
    let to_token = caps.get(3)?.as_str().trim();

    let from_crypto = resolve_crypto(from_token);
    let to_crypto = resolve_crypto(to_token);
    let from_fiat = resolve_fiat(from_token);
    let to_fiat = resolve_fiat(to_token);

    if let Some(from) = from_crypto {
        if let Some(to) = to_crypto.clone().or(to_fiat.clone()) {
            return Some(Intent::Crypto { amount, from, to });
        }
    }

    if let Some(from) = from_fiat {
        if let Some(to) = to_crypto {
            return Some(Intent::Crypto { amount, from, to });
        }

        if let Some(to) = to_fiat {
            return Some(Intent::Currency { amount, from, to });
        }
    }

    None
}
