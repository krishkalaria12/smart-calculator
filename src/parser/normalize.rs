use regex::Regex;
use std::sync::LazyLock;

pub struct AppRegexes {
    // --- General ---
    pub whitespace: Regex,
    pub prefixes: Regex,

    // --- Currency & Conversion ---
    pub conversion_pattern: Regex,
    pub currency_symbol_prefix: Regex,

    // --- Time: Relative & Phrases ---
    pub time_relative_simple: Regex,
    pub time_relative_complex: Regex,
    pub time_relative_phrase: Regex,
    pub time_duration_offset: Regex,
    pub time_in_duration: Regex,

    // --- Time: Unix & Technical ---
    pub unix_timestamp_raw: Regex,
    pub unix_timestamp_explicit: Regex,
    pub to_unix_conversion: Regex,
    pub iso_date: Regex,

    // --- Time: Natural Language Queries ---
    pub current_date_query: Regex,
    pub today_query: Regex,
    pub days_between: Regex,

    // -- Math --
    pub math_prefix_cleanup: Regex,
    pub math_the_cleanup: Regex,
    pub math_wrappers: Vec<(Regex, &'static str)>,
    pub math_word_operators: Vec<(Regex, &'static str)>,
}

pub static REGEXES: LazyLock<AppRegexes> = LazyLock::new(|| {
    AppRegexes {
        // --- General ---
        whitespace: Regex::new(r"\s+").expect("Invalid whitespace regex"),

        prefixes: Regex::new(r"(?i)^(?:convert|how much is|what(?:'s|s| is))\s+")
            .expect("Invalid prefix removal regex"),

        // --- Currency & Conversion ---
        conversion_pattern: Regex::new(
            r"(?i)^([\d.,]+)?\s*([a-zA-Z$€£¥₹₩₽₺₦₵₪฿]+(?:\s+[a-zA-Z]+)?)\s+(?:to|in|into|as|=)\s+([a-zA-Z$€£¥₹₩₽₺₦₵₪฿]+(?:\s+[a-zA-Z]+)?)$",
        ).expect("Invalid currency conversion pattern"),

        currency_symbol_prefix: Regex::new(r"^([$€£¥₹₩₽₺₦₵₪฿])\s*([\d.,]+)")
            .expect("Invalid currency symbol-prefix regex"),

        // --- Time: Relative & Phrases ---
        time_relative_simple: Regex::new(r"(?i)^(today|now|tomorrow|yesterday)$")
            .expect("Invalid simple time regex"),

        time_relative_complex: Regex::new(r"(?i)^(date|day after tomorrow|day before yesterday)$")
            .expect("Invalid complex relative time regex"),

        time_relative_phrase: Regex::new(
            r"(?i)^(next|last|this)\s+(monday|tuesday|wednesday|thursday|friday|saturday|sunday|week|month|year)$"
        ).expect("Invalid relative date phrase regex"),

        time_duration_offset: Regex::new(
            r"(?i)^(\d+)\s+(days?|weeks?|months?|years?|hours?|minutes?|seconds?)\s+(from now|ago|from today|from tomorrow)$"
        ).expect("Invalid duration offset regex"),

        time_in_duration: Regex::new(r"(?i)^in\s+(\d+)\s+(days?|weeks?|months?|years?|hours?|minutes?|seconds?)$")
            .expect("Invalid 'in duration' regex"),

        // --- Time: Unix & Technical ---
        unix_timestamp_raw: Regex::new(r"(?i)^(?:unix\s+)?(?:timestamp\s+)?(\d{10,13}|0)$")
            .expect("Invalid raw unix timestamp regex"),

        unix_timestamp_explicit: Regex::new(r"(?i)^(?:unix|timestamp|epoch)\s+(\d{10,13})$")
            .expect("Invalid explicit unix regex"),

        to_unix_conversion: Regex::new(r"(?i)^(.+?)\s+(?:to|in)\s+(?:unix|timestamp|epoch)$")
            .expect("Invalid 'to unix' conversion regex"),

        iso_date: Regex::new(r"^\d{4}-\d{2}-\d{2}(?:T\d{2}:\d{2})?")
            .expect("Invalid ISO date regex"),

        // --- Time: Natural Language Queries ---
        current_date_query: Regex::new(r"(?i)^(?:what(?:'s| is) )?(?:the )?(?:current )?date(?: today)?$")
            .expect("Invalid current date query regex"),

        today_query: Regex::new(
            r"(?i)^(?:(?:what(?:'s|s| is))\s+today|(?:what(?:'s|s| is))\s+today'?s date|what day is it|what date is .+|todays date|today'?s date|date today|current date|what day is .+|when is .+)$"
        ).expect("Invalid natural language today query regex"),

        days_between: Regex::new(
            r"(?i)^(?:days?\s+)?(?:between|from)\s+(.+?)\s+(?:to|and|until)\s+(.+)$",
        )
            .expect("Invalid days-between regex"),

        // -- Math --
        math_prefix_cleanup: Regex::new(r"(?i)^(?:(?:what(?:'s|s| is))\s+the\s+|(?:what(?:'s|s| is))\s+|calculate\s+)").unwrap(),
        math_the_cleanup: Regex::new(r"(?i)^the\s+").unwrap(),

        math_wrappers: vec![
            (Regex::new(r"(?i)^square root of (.+)$").unwrap(), "sqrt($1)"),
            (Regex::new(r"(?i)^cube root of (.+)$").unwrap(), "cbrt($1)"),
            (Regex::new(r"(?i)^log of (.+)$").unwrap(), "log($1)"),
            (Regex::new(r"(?i)^sine of (.+)$").unwrap(), "sin($1)"),
            (Regex::new(r"(?i)^cosine of (.+)$").unwrap(), "cos($1)"),
            (Regex::new(r"(?i)^tangent of (.+)$").unwrap(), "tan($1)"),
            (Regex::new(r"(?i)^absolute value of (.+)$").unwrap(), "abs($1)"),
            (Regex::new(r"(?i)^factorial of (.+)$").unwrap(), "($1)!"),
            (Regex::new(r"(?i)^half of (.+)$").unwrap(), "($1) / 2"),
            (Regex::new(r"(?i)^double (.+)$").unwrap(), "2 * ($1)"),
            (Regex::new(r"(?i)^triple (.+)$").unwrap(), "3 * ($1)"),
            (Regex::new(r"(?i)^one third of (.+)$").unwrap(), "($1) / 3"),
            (Regex::new(r"(?i)^one quarter of (.+)$").unwrap(), "($1) / 4"),
            (Regex::new(r"(?i)^remainder of (.+?) divided by (.+)$").unwrap(), "($1) % ($2)"),
            (Regex::new(r"(?i)^(.+?) to the power of (.+)$").unwrap(), "($1) ^ ($2)"),
            (Regex::new(r"(?i)^(.+?) raised to (.+)$").unwrap(), "($1) ^ ($2)"),
            (Regex::new(r"(?i)^(.+?) squared$").unwrap(), "($1) ^ 2"),
            (Regex::new(r"(?i)^(.+?) cubed$").unwrap(), "($1) ^ 3"),
            (Regex::new(r"(?i)^(\d+(?:\.\d+)?) percent of (.+)$").unwrap(), "($1%) * ($2)"),
            (Regex::new(r"(?i)^(.+?%) of (.+)$").unwrap(), "($1) * ($2)"),
        ],

        math_word_operators: vec![
            (Regex::new(r"(?i)\bdivided by\b").unwrap(), "/"),
            (Regex::new(r"(?i)\btimes\b").unwrap(), "*"),
            (Regex::new(r"(?i)\bplus\b").unwrap(), "+"),
            (Regex::new(r"(?i)\bminus\b").unwrap(), "-"),
        ],
    }
});

pub fn date_intent_regexes() -> [&'static Regex; 12] {
    [
        &REGEXES.time_relative_simple,
        &REGEXES.time_relative_complex,
        &REGEXES.time_relative_phrase,
        &REGEXES.time_duration_offset,
        &REGEXES.time_in_duration,
        &REGEXES.unix_timestamp_raw,
        &REGEXES.unix_timestamp_explicit,
        &REGEXES.to_unix_conversion,
        &REGEXES.iso_date,
        &REGEXES.current_date_query,
        &REGEXES.today_query,
        &REGEXES.days_between,
    ]
}

pub fn normalize_whitespace(input: &str) -> String {
    let trimmed = input.trim();
    REGEXES.whitespace.replace_all(trimmed, " ").into_owned()
}

pub fn normalize_conversion_input(input: &str) -> String {
    let normalized_ws = normalize_whitespace(input);

    let normalized = REGEXES.prefixes.replace(&normalized_ws, "").into_owned();

    REGEXES
        .currency_symbol_prefix
        .replace(&normalized, "$2 $1")
        .into_owned()
}
