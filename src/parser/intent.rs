use crate::{
    data::currencies::{resolve_crypto, resolve_fiat},
    data::timezone::resolve_timezone,
    data::units::lookup_unit,
    types::Intent,
};

use super::normalize::{
    date_intent_regexes, normalize_conversion_input, normalize_time_place, normalize_whitespace,
    REGEXES,
};

pub fn detect_intent(input: &str) -> Intent {
    let trimmed = normalize_whitespace(input);
    if trimmed.is_empty() {
        return Intent::Math {
            expression: "0".to_string(),
        };
    }

    if let Some(intent) = try_time_intent(trimmed.clone()) {
        return intent;
    }

    if let Some(intent) = try_date_intent(trimmed.clone()) {
        return intent;
    }

    if let Some(intent) = try_currency_or_crypto_intent(&trimmed) {
        return intent;
    }

    if let Some(intent) = try_unit_intent(&trimmed) {
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

fn try_unit_intent(input: &str) -> Option<Intent> {
    let normalized = normalize_conversion_input(input);
    let caps = REGEXES
        .unit_pattern
        .captures(&normalized)
        .or_else(|| REGEXES.unit_pattern_no_space.captures(&normalized))?;

    let amount = caps.get(1)?.as_str().replace(',', "").parse::<f64>().ok()?;
    let from_token = caps.get(2)?.as_str().trim().to_lowercase();
    let to_token = caps.get(3)?.as_str().trim().to_lowercase();

    let from_unit = lookup_unit(&from_token);
    let to_unit = lookup_unit(&to_token);

    if let (Some(from), Some(to)) = (from_unit, to_unit) {
        if from.category_name == to.category_name {
            return Some(Intent::Unit {
                amount,
                from: from_token,
                to: to_token,
                category: from.category_name.to_string(),
            });
        }
    }

    if resolve_fiat(&from_token).is_some() || resolve_crypto(&from_token).is_some() {
        return None;
    }

    if resolve_fiat(&to_token).is_some() || resolve_crypto(&to_token).is_some() {
        return None;
    }

    None
}

pub fn try_time_intent(input: String) -> Option<Intent> {
    let normalized = normalize_whitespace(&input);

    for pattern in [
        &REGEXES.time_in_primary,
        &REGEXES.time_in_secondary,
        &REGEXES.time_in_now,
    ] {
        if let Some(caps) = pattern.captures(&normalized) {
            let place = normalize_time_place(caps.get(1)?.as_str());
            let tz = resolve_timezone(&place).unwrap_or(place);
            return Some(Intent::Time {
                query: input,
                from: None,
                to: Some(tz),
                time: None,
            });
        }
    }

    if let Some(caps) = REGEXES.time_explicit_convert.captures(&normalized) {
        let explicit_time = caps.get(1)?.as_str().trim().to_string();
        let from = normalize_time_place(caps.get(2)?.as_str());
        let to = normalize_time_place(caps.get(3)?.as_str());
        let from_tz = resolve_timezone(&from)?;
        let to_tz = resolve_timezone(&to)?;

        return Some(Intent::Time {
            query: input,
            from: Some(from_tz),
            to: Some(to_tz),
            time: Some(explicit_time),
        });
    }

    if let Some(caps) = REGEXES.time_convert.captures(&normalized) {
        let from = normalize_time_place(caps.get(1)?.as_str());
        let to = normalize_time_place(caps.get(2)?.as_str());
        let from_tz = resolve_timezone(&from);
        let to_tz = resolve_timezone(&to);

        if from_tz.is_some() || to_tz.is_some() {
            return Some(Intent::Time {
                query: input,
                from: Some(from_tz.unwrap_or(from)),
                to: Some(to_tz.unwrap_or(to)),
                time: None,
            });
        }
    }

    if REGEXES.time_now_query.is_match(&normalized) {
        return Some(Intent::Time {
            query: input,
            from: None,
            to: None,
            time: None,
        });
    }

    if normalized.to_lowercase().starts_with("time ") {
        return Some(Intent::Time {
            query: input,
            from: None,
            to: None,
            time: None,
        });
    }

    if let Some(caps) = REGEXES.time_place_suffix.captures(&normalized) {
        let place = normalize_time_place(caps.get(1)?.as_str());
        if let Some(tz) = resolve_timezone(&place) {
            return Some(Intent::Time {
                query: input,
                from: None,
                to: Some(tz),
                time: None,
            });
        }
    }

    let place = normalize_time_place(&normalized);
    resolve_timezone(&place).map(|tz| Intent::Time {
        query: input,
        from: None,
        to: Some(tz),
        time: None,
    })
}

pub fn try_date_intent(input: String) -> Option<Intent> {
    let normalized = normalize_whitespace(&input)
        .to_lowercase()
        .trim_end_matches(['?', '!', '.'])
        .to_string();
    let is_date_related = date_intent_regexes()
        .iter()
        .any(|pattern| pattern.is_match(&normalized));

    if is_date_related {
        return Some(Intent::Date { query: input });
    }

    None
}
