use chrono::{
    DateTime, Datelike, Duration, Local, Months, NaiveDate, SecondsFormat, TimeZone, Utc,
};
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::{
    evaluators::error::{EvaluatorError, Result},
    parser::REGEXES,
    types::{AnswerType, CalculatorResult, RelativeDirection, ResultType},
};

pub const DAY_NAMES: [&str; 7] = [
    "sunday",
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
];

fn normalize_day_token<'a>(token: &'a str) -> Option<&'a str> {
    match token {
        "mon" => Some("monday"),
        "tue" | "tues" => Some("tuesday"),
        "wed" => Some("wednesday"),
        "thu" | "thur" | "thurs" => Some("thursday"),
        "fri" => Some("friday"),
        "sat" => Some("saturday"),
        "sun" => Some("sunday"),
        "monday" | "tuesday" | "wednesday" | "thursday" | "friday" | "saturday"
        | "sunday" => Some(token),
        _ => None,
    }
}

pub fn evaluate_date(query: String) -> Result<CalculatorResult> {
    let normalized = query
        .trim()
        .to_lowercase()
        .trim_end_matches(['?', '!', '.'])
        .to_string();
    let now = Local::now();

    if is_current_date_query(&normalized) {
        return Ok(format_date_result(query, now, None));
    }

    if let Some(date) = parse_simple_relative(&normalized, now) {
        return Ok(format_date_result(query, date, None));
    }

    if let Some(date) = parse_relative_phrase(&normalized, now) {
        return Ok(format_date_result(query, date, None));
    }

    if let Some(date) = parse_target_date_query(&normalized, now) {
        return Ok(format_date_result(query, date, None));
    }

    if let Some(date) = parse_weekday_only(&normalized, now) {
        return Ok(format_date_result(query, date, None));
    }

    if let Some(date) = parse_named_date_anchor(&normalized, now) {
        return Ok(format_date_result(query, date, None));
    }

    if let Some(date) = parse_compound_relative(&normalized, now) {
        return Ok(format_date_result(query, date, None));
    }

    if let Some(date) = parse_duration_offset(&normalized, now) {
        return Ok(format_date_result(query, date, None));
    }

    if let Some(date) = parse_in_duration(&normalized, now) {
        return Ok(format_date_result(query, date, None));
    }

    if let Some(date) = parse_unix_timestamp(&normalized) {
        return Ok(format_date_result(query, date, None));
    }

    if let Some(date_str) = parse_to_unix_source(&normalized) {
        if let Some(date) = parse_flexible_date(date_str, now) {
            let unix = date.timestamp();
            return Ok(CalculatorResult {
                input: query,
                res_type: ResultType::Date,
                result: AnswerType::Text(unix.to_string()),
                formatted: unix.to_string(),
                metadata: Some(HashMap::from([(
                    "iso".to_string(),
                    json!(date.to_rfc3339_opts(SecondsFormat::Millis, true)),
                )])),
            });
        }
    }

    if let Some((start_str, end_str)) = parse_days_between_parts(&normalized) {
        let start = parse_flexible_date(start_str, now);
        let end = parse_flexible_date(end_str, now);

        if let (Some(start), Some(end)) = (start, end) {
            let diff = end.signed_duration_since(start).num_days().abs();
            return Ok(CalculatorResult {
                input: query,
                res_type: ResultType::Date,
                result: AnswerType::Text(diff.to_string()),
                formatted: format!("{diff} days"),
                metadata: None,
            });
        }
    }

    if let Some(date) = parse_flexible_date(&normalized, now) {
        return Ok(format_date_result(query, date, None));
    }

    Err(EvaluatorError::DateEvaluationFailed(
        "could not parse date query".to_string(),
    ))
}

fn is_current_date_query(input: &str) -> bool {
    input == "today"
        || input == "now"
        || input == "date"
        || REGEXES.today_query.is_match(input)
        || REGEXES.current_date_query.is_match(input)
}

fn parse_simple_relative(input: &str, now: DateTime<Local>) -> Option<DateTime<Local>> {
    match input {
        "today" | "the today" => Some(now),
        "now" | "the now" => Some(now),
        "tomorrow" => Some(now + Duration::days(1)),
        "tmr" | "tmrw" | "the tomorrow" => Some(now + Duration::days(1)),
        "yesterday" => Some(now - Duration::days(1)),
        "yday" | "the yesterday" => Some(now - Duration::days(1)),
        "day after tomorrow" => Some(now + Duration::days(2)),
        "day after tmr" => Some(now + Duration::days(2)),
        "day before yesterday" => Some(now - Duration::days(2)),
        "day before yday" => Some(now - Duration::days(2)),
        _ => None,
    }
}

fn parse_relative_phrase(input: &str, now: DateTime<Local>) -> Option<DateTime<Local>> {
    let caps = REGEXES.time_relative_phrase.captures(input)?;

    let direction = RelativeDirection::from_str(caps.get(1)?.as_str())?;
    let raw_target = caps.get(2)?.as_str().to_lowercase();
    let target = normalize_day_token(&raw_target).unwrap_or(raw_target.as_str());

    match target {
        "week" => {
            let amount = match direction {
                RelativeDirection::Next => 1,
                RelativeDirection::Last => -1,
                RelativeDirection::This => 0,
            };
            Some(apply_offset(now, amount, "week"))
        }
        "weekend" => {
            let friday_idx = DAY_NAMES
                .iter()
                .position(|day| *day == "friday")? as i32;
            Some(get_relative_day(now, friday_idx, direction))
        }
        "month" => {
            let amount = match direction {
                RelativeDirection::Next => 1,
                RelativeDirection::Last => -1,
                RelativeDirection::This => 0,
            };
            Some(apply_offset(now, amount, "month"))
        }
        "year" => {
            let amount = match direction {
                RelativeDirection::Next => 1,
                RelativeDirection::Last => -1,
                RelativeDirection::This => 0,
            };
            Some(apply_offset(now, amount, "year"))
        }
        _ => {
            let target_day = DAY_NAMES.iter().position(|day| *day == target)? as i32;
            Some(get_relative_day(now, target_day, direction))
        }
    }
}

fn parse_duration_offset(input: &str, now: DateTime<Local>) -> Option<DateTime<Local>> {
    let caps = REGEXES.time_duration_offset.captures(input)?;

    let amount = caps.get(1)?.as_str().parse::<i32>().ok()?;
    let unit = caps.get(2)?.as_str();
    let direction = caps.get(3)?.as_str().to_lowercase();

    let base = match direction.as_str() {
        "from tomorrow" => now + Duration::days(1),
        "from yesterday" => now - Duration::days(1),
        _ => now,
    };

    let signed_amount = if direction == "ago" { -amount } else { amount };
    Some(apply_offset(base, signed_amount, unit))
}

fn parse_in_duration(input: &str, now: DateTime<Local>) -> Option<DateTime<Local>> {
    let caps = REGEXES.time_in_duration.captures(input)?;

    let amount = caps.get(1)?.as_str().parse::<i32>().ok()?;
    let unit = caps.get(2)?.as_str();

    Some(apply_offset(now, amount, unit))
}

fn parse_unix_timestamp(input: &str) -> Option<DateTime<Local>> {
    let ts_str = REGEXES
        .unix_timestamp_explicit
        .captures(input)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .or_else(|| {
            REGEXES
                .unix_timestamp_raw
                .captures(input)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        })?;

    let ts = ts_str.parse::<i64>().ok()?;
    let dt_utc = if ts_str.len() > 10 {
        Utc.timestamp_millis_opt(ts).single()
    } else {
        Utc.timestamp_opt(ts, 0).single()
    }?;

    Some(DateTime::from(dt_utc))
}

fn parse_to_unix_source(input: &str) -> Option<&str> {
    let caps = REGEXES.to_unix_conversion.captures(input)?;
    Some(caps.get(1)?.as_str().trim())
}

fn parse_days_between_parts(input: &str) -> Option<(&str, &str)> {
    let caps = REGEXES.days_between.captures(input)?;
    let start = caps.get(1)?.as_str().trim();
    let end = caps.get(2)?.as_str().trim();
    Some((start, end))
}

fn parse_target_date_query(input: &str, now: DateTime<Local>) -> Option<DateTime<Local>> {
    let caps = REGEXES.date_target_query.captures(input)?;
    let target = caps
        .get(1)
        .or_else(|| caps.get(2))?
        .as_str()
        .trim()
        .to_lowercase();
    parse_flexible_date(&target, now)
}

fn parse_weekday_only(input: &str, now: DateTime<Local>) -> Option<DateTime<Local>> {
    let caps = REGEXES.weekday_only.captures(input)?;
    let day = caps.get(1)?.as_str().to_lowercase();
    let day = normalize_day_token(&day).unwrap_or(day.as_str());
    let target_day = DAY_NAMES.iter().position(|candidate| *candidate == day)? as i32;
    Some(get_relative_day(now, target_day, RelativeDirection::This))
}

fn parse_named_date_anchor(input: &str, now: DateTime<Local>) -> Option<DateTime<Local>> {
    let caps = REGEXES.date_named_anchor.captures(input)?;
    let anchor = caps.get(1)?.as_str().to_lowercase();
    let unit = caps.get(2)?.as_str().to_lowercase();

    let is_end = anchor == "end";

    match unit.as_str() {
        "week" => {
            if !is_end {
                let sunday_idx = DAY_NAMES.iter().position(|day| *day == "sunday")? as i32;
                Some(get_relative_day(now, sunday_idx, RelativeDirection::This))
            } else {
                let saturday_idx = DAY_NAMES
                    .iter()
                    .position(|day| *day == "saturday")? as i32;
                Some(get_relative_day(now, saturday_idx, RelativeDirection::This))
            }
        }
        "month" => {
            let current = now.date_naive();
            if !is_end {
                let start = current.with_day(1)?;
                let naive_dt = start.and_hms_opt(0, 0, 0)?;
                Local.from_local_datetime(&naive_dt).single()
            } else {
                let next_month = now + Months::new(1);
                let first_of_next = next_month.date_naive().with_day(1)?;
                let end = first_of_next.checked_sub_days(chrono::Days::new(1))?;
                let naive_dt = end.and_hms_opt(0, 0, 0)?;
                Local.from_local_datetime(&naive_dt).single()
            }
        }
        "year" => {
            let current = now.date_naive();
            if !is_end {
                let start = NaiveDate::from_ymd_opt(current.year(), 1, 1)?;
                let naive_dt = start.and_hms_opt(0, 0, 0)?;
                Local.from_local_datetime(&naive_dt).single()
            } else {
                let end = NaiveDate::from_ymd_opt(current.year(), 12, 31)?;
                let naive_dt = end.and_hms_opt(0, 0, 0)?;
                Local.from_local_datetime(&naive_dt).single()
            }
        }
        _ => None,
    }
}

fn parse_compound_relative(input: &str, now: DateTime<Local>) -> Option<DateTime<Local>> {
    let caps = REGEXES.date_compound_relative.captures(input)?;
    let unit = caps.get(1)?.as_str().to_lowercase();
    let relation = caps.get(2)?.as_str().to_lowercase();

    let amount = match relation.as_str() {
        "after next" => 2,
        "before last" => -2,
        _ => return None,
    };

    Some(apply_offset(now, amount, &unit))
}

fn get_relative_day(
    now: DateTime<Local>,
    target_day: i32,
    direction: RelativeDirection,
) -> DateTime<Local> {
    let current_day = now.weekday().num_days_from_sunday() as i32;

    let diff = match direction {
        RelativeDirection::Next => {
            let mut d = (target_day - current_day + 7) % 7;
            if d == 0 {
                d = 7;
            }
            d
        }
        RelativeDirection::Last => {
            let mut d = -((current_day - target_day + 7) % 7);
            if d == 0 {
                d = -7;
            }
            d
        }
        RelativeDirection::This => target_day - current_day,
    };

    now + Duration::days(diff as i64)
}

fn apply_offset(base: DateTime<Local>, amount: i32, unit: &str) -> DateTime<Local> {
    let unit = unit.to_lowercase();
    let unit = unit.trim_end_matches('s');

    match unit {
        "second" => base + Duration::seconds(amount as i64),
        "minute" => base + Duration::minutes(amount as i64),
        "hour" => base + Duration::hours(amount as i64),
        "day" => base + Duration::days(amount as i64),
        "week" => base + Duration::weeks(amount as i64),
        "month" => {
            if amount >= 0 {
                base + Months::new(amount as u32)
            } else {
                base - Months::new(amount.unsigned_abs())
            }
        }
        "year" => {
            let months = amount.unsigned_abs() * 12;
            if amount >= 0 {
                base + Months::new(months)
            } else {
                base - Months::new(months)
            }
        }
        _ => base,
    }
}

fn parse_flexible_date(input: &str, now: DateTime<Local>) -> Option<DateTime<Local>> {
    let normalized = input
        .trim()
        .to_lowercase()
        .trim_end_matches(['?', '!', '.'])
        .to_string();

    let input = normalized.as_str();

    if let Some(simple) = parse_simple_relative(input, now) {
        return Some(simple);
    }

    if input == "today" || input == "now" || input == "date" {
        return Some(now);
    }

    if let Some(date) = parse_relative_phrase(input, now) {
        return Some(date);
    }

    if let Some(date) = parse_target_date_query(input, now) {
        return Some(date);
    }

    if let Some(date) = parse_weekday_only(input, now) {
        return Some(date);
    }

    if let Some(date) = parse_named_date_anchor(input, now) {
        return Some(date);
    }

    if let Some(date) = parse_compound_relative(input, now) {
        return Some(date);
    }

    if REGEXES.iso_date.is_match(input) {
        if let Ok(dt) = DateTime::parse_from_rfc3339(input) {
            return Some(DateTime::from(dt));
        }

        if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M") {
            if let Some(local_dt) = Local.from_local_datetime(&naive_dt).single() {
                return Some(local_dt);
            }
        }
    }

    for format in [
        "%Y-%m-%d",
        "%d-%m-%Y",
        "%m/%d/%Y",
        "%d/%m/%Y",
        "%B %d %Y",
        "%b %d %Y",
        "%B %d, %Y",
        "%b %d, %Y",
        "%d %B %Y",
        "%d %b %Y",
    ] {
        if let Ok(naive_date) = NaiveDate::parse_from_str(input, format)
            && let Some(naive_dt) = naive_date.and_hms_opt(0, 0, 0)
        {
            if let Some(local_dt) = Local.from_local_datetime(&naive_dt).single() {
                return Some(local_dt);
            }
        }
    }

    None
}

fn format_date(date: DateTime<Local>) -> String {
    date.format("%A, %B %e, %Y, %I:%M %p").to_string()
}

fn format_date_result(
    query: String,
    date: DateTime<Local>,
    extra_metadata: Option<HashMap<String, Value>>,
) -> CalculatorResult {
    let mut metadata = HashMap::new();
    metadata.insert(
        "iso".to_string(),
        json!(date.to_rfc3339_opts(SecondsFormat::Millis, true)),
    );
    metadata.insert("unix".to_string(), json!(date.timestamp()));
    metadata.insert(
        "dayOfWeek".to_string(),
        json!(DAY_NAMES[date.weekday().num_days_from_sunday() as usize]),
    );

    if let Some(extra) = extra_metadata {
        metadata.extend(extra);
    }

    CalculatorResult {
        input: query,
        res_type: ResultType::Date,
        result: AnswerType::Text(date.to_rfc3339_opts(SecondsFormat::Millis, true)),
        formatted: format_date(date),
        metadata: Some(metadata),
    }
}
