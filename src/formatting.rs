use fixed_decimal::Decimal as FixedDecimal;
use icu::decimal::DecimalFormatter;
use icu::locale::Locale;
use icu::locale::locale;

pub const MAX_DISPLAY_FRACTION_DIGITS: u8 = 3;

pub fn normalize_locale(locale: Option<String>) -> String {
    locale
        .unwrap_or_else(|| "en-US".to_string())
        .trim()
        .replace('_', "-")
}

pub fn format_display_number(value: f64, locale_str: &str, max_fraction_digits: u8) -> String {
    format_localized_number(value, locale_str, max_fraction_digits)
        .unwrap_or_else(|| compact_decimal_string(value, max_fraction_digits))
}

fn format_localized_number(
    value: f64,
    locale_str: &str,
    max_fraction_digits: u8,
) -> Option<String> {
    let locale = locale_str
        .parse::<Locale>()
        .ok()
        .unwrap_or(locale!("en-US"));
    let formatter = DecimalFormatter::try_new(locale.into(), Default::default()).ok()?;
    let decimal = compact_decimal_string(value, max_fraction_digits)
        .parse::<FixedDecimal>()
        .ok()?;

    Some(format!("{}", formatter.format(&decimal)))
}

fn compact_decimal_string(value: f64, max_fraction_digits: u8) -> String {
    if !value.is_finite() {
        return value.to_string();
    }

    let precision = usize::from(max_fraction_digits);
    let mut formatted = format!("{value:.precision$}");

    if formatted.contains('.') {
        while formatted.ends_with('0') {
            formatted.pop();
        }

        if formatted.ends_with('.') {
            formatted.pop();
        }
    }

    if formatted == "-0" {
        formatted.clear();
        formatted.push('0');
    }

    formatted
}
