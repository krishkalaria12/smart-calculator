use chrono::{DateTime, Duration, LocalResult, NaiveDateTime, SecondsFormat, TimeZone, Utc};
use chrono_tz::Tz;
use serde_json::{Value, json};
use std::{collections::HashMap, str::FromStr, sync::LazyLock};

use crate::{
    data::timezone::resolve_timezone,
    evaluators::error::{EvaluatorError, Result},
    types::{AnswerType, CalculatorResult, ResultType},
};

static TIME_EXPRESSION: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?i)^(\d{1,2})(?::(\d{2}))?\s*(am|pm)?$")
        .expect("Invalid time expression regex")
});

pub fn evaluate_time(
    query: String,
    from_tz: Option<String>,
    to_tz: Option<String>,
    explicit_time: Option<String>,
    local_tz: Option<String>,
) -> Result<CalculatorResult> {
    let now = Utc::now();
    let user_tz = detect_user_timezone(local_tz);

    if from_tz.is_none() && to_tz.is_none() {
        let formatted = format_time_in_zone(now, &user_tz);
        return Ok(CalculatorResult {
            input: query,
            res_type: ResultType::Time,
            result: AnswerType::Text(formatted.clone()),
            formatted,
            metadata: Some(HashMap::from([
                ("timezone".to_string(), json!(user_tz.name())),
                (
                    "iso".to_string(),
                    json!(now.to_rfc3339_opts(SecondsFormat::Millis, true)),
                ),
            ])),
        });
    }

    if let Some(to_name) = to_tz.as_ref()
        && from_tz.is_none()
    {
        let resolved_to = parse_timezone(&to_name).map_err(|_| {
            EvaluatorError::TimeEvaluationFailed(format!("unknown timezone: '{to_name}'"))
        })?;
        let formatted = format_time_in_zone(now, &resolved_to);

        return Ok(CalculatorResult {
            input: query,
            res_type: ResultType::Time,
            result: AnswerType::Text(formatted.clone()),
            formatted,
            metadata: Some(HashMap::from([
                ("timezone".to_string(), json!(resolved_to.name())),
                (
                    "iso".to_string(),
                    json!(now.to_rfc3339_opts(SecondsFormat::Millis, true)),
                ),
            ])),
        });
    }

    if let (Some(from_name), Some(to_name)) = (from_tz, to_tz) {
        let resolved_from = parse_timezone(&from_name).map_err(|_| {
            EvaluatorError::TimeEvaluationFailed(format!(
                "cannot resolve timezone conversion: '{from_name}' -> '{to_name}'"
            ))
        })?;
        let resolved_to = parse_timezone(&to_name).map_err(|_| {
            EvaluatorError::TimeEvaluationFailed(format!(
                "cannot resolve timezone conversion: '{from_name}' -> '{to_name}'"
            ))
        })?;

        let source_date = if let Some(time_expression) = explicit_time.as_deref() {
            build_date_in_zone(time_expression, &resolved_from, now)?
        } else {
            now
        };

        let from_time = if let Some(time_expression) = explicit_time.as_deref() {
            format!(
                "{time_expression} ({})",
                get_short_tz_name(source_date, &resolved_from)
            )
        } else {
            format_time_in_zone(source_date, &resolved_from)
        };
        let to_time = format_time_in_zone(source_date, &resolved_to);

        let mut metadata = HashMap::<String, Value>::from([
            (
                "iso".to_string(),
                json!(source_date.to_rfc3339_opts(SecondsFormat::Millis, true)),
            ),
            (
                "from".to_string(),
                json!({
                    "timezone": resolved_from.name(),
                    "time": from_time,
                }),
            ),
            (
                "to".to_string(),
                json!({
                    "timezone": resolved_to.name(),
                    "time": to_time,
                }),
            ),
        ]);

        if let Some(time_expression) = explicit_time {
            metadata.insert("explicitTime".to_string(), json!(time_expression));
        }

        return Ok(CalculatorResult {
            input: query,
            res_type: ResultType::Time,
            result: AnswerType::Text(to_time.clone()),
            formatted: to_time,
            metadata: Some(metadata),
        });
    }

    Err(EvaluatorError::TimeEvaluationFailed(
        "could not parse time query".to_string(),
    ))
}

fn detect_user_timezone(configured: Option<String>) -> Tz {
    configured
        .as_deref()
        .and_then(|value| parse_timezone(value).ok())
        .or_else(|| {
            iana_time_zone::get_timezone()
                .ok()
                .and_then(|value| parse_timezone(&value).ok())
        })
        .unwrap_or(chrono_tz::UTC)
}

fn parse_timezone(value: &str) -> std::result::Result<Tz, ()> {
    let candidate = resolve_timezone(value).unwrap_or_else(|| value.trim().to_string());
    Tz::from_str(&candidate).map_err(|_| ())
}

fn build_date_in_zone(
    time_expression: &str,
    timezone: &Tz,
    anchor: DateTime<Utc>,
) -> Result<DateTime<Utc>> {
    let (hour, minute) = parse_time_expression(time_expression)?;
    let local_date = anchor.with_timezone(timezone).date_naive();
    let naive = local_date.and_hms_opt(hour, minute, 0).ok_or_else(|| {
        EvaluatorError::TimeEvaluationFailed(format!("invalid time: '{time_expression}'"))
    })?;

    resolve_local_datetime(*timezone, naive)
        .map(|dt| dt.with_timezone(&Utc))
        .ok_or_else(|| {
            EvaluatorError::TimeEvaluationFailed(format!("invalid time: '{time_expression}'"))
        })
}

fn resolve_local_datetime(timezone: Tz, naive: NaiveDateTime) -> Option<DateTime<Tz>> {
    match timezone.from_local_datetime(&naive) {
        LocalResult::Single(dt) => Some(dt),
        LocalResult::Ambiguous(first, _) => Some(first),
        LocalResult::None => {
            for minutes in 1..=180 {
                let shifted = naive + Duration::minutes(minutes);
                match timezone.from_local_datetime(&shifted) {
                    LocalResult::Single(dt) => return Some(dt),
                    LocalResult::Ambiguous(first, _) => return Some(first),
                    LocalResult::None => {}
                }
            }
            None
        }
    }
}

fn parse_time_expression(value: &str) -> Result<(u32, u32)> {
    let trimmed = value.trim().to_lowercase();
    if trimmed == "midnight" {
        return Ok((0, 0));
    }
    if trimmed == "noon" {
        return Ok((12, 0));
    }

    let captures = TIME_EXPRESSION
        .captures(&trimmed)
        .ok_or_else(|| EvaluatorError::TimeEvaluationFailed(format!("invalid time: '{value}'")))?;

    let mut hour = captures
        .get(1)
        .and_then(|part| part.as_str().parse::<u32>().ok())
        .ok_or_else(|| EvaluatorError::TimeEvaluationFailed(format!("invalid time: '{value}'")))?;
    let minute = captures
        .get(2)
        .and_then(|part| part.as_str().parse::<u32>().ok())
        .unwrap_or(0);
    let meridiem = captures.get(3).map(|part| part.as_str().to_lowercase());

    if minute > 59 {
        return Err(EvaluatorError::TimeEvaluationFailed(format!(
            "invalid time: '{value}'"
        )));
    }

    if let Some(meridiem) = meridiem {
        if hour == 0 || hour > 12 {
            return Err(EvaluatorError::TimeEvaluationFailed(format!(
                "invalid time: '{value}'"
            )));
        }

        if meridiem == "am" && hour == 12 {
            hour = 0;
        } else if meridiem == "pm" && hour < 12 {
            hour += 12;
        }
    } else if hour > 23 {
        return Err(EvaluatorError::TimeEvaluationFailed(format!(
            "invalid time: '{value}'"
        )));
    }

    Ok((hour, minute))
}

fn format_time_in_zone(date: DateTime<Utc>, timezone: &Tz) -> String {
    let local = date.with_timezone(timezone);
    format!(
        "{}, {} ({})",
        local.format("%-I:%M:%S %p"),
        local.format("%a, %b %-d, %Y"),
        local.format("%Z")
    )
}

fn get_short_tz_name(date: DateTime<Utc>, timezone: &Tz) -> String {
    date.with_timezone(timezone).format("%Z").to_string()
}
