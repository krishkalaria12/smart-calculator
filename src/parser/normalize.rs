use regex::Regex;
use std::sync::LazyLock;

pub struct AppRegexes {
    pub whitespace: Regex,
    pub conversion_pattern: Regex,
    pub prefixes: Regex,
    pub currency_symbol_prefix: Regex,
}

pub static REGEXES: LazyLock<AppRegexes> = LazyLock::new(|| {
    AppRegexes {
    whitespace: Regex::new(r"\s+").expect("valid whitespace regex"),
    conversion_pattern: Regex::new(
        r"(?i)^([\d.,]+)?\s*([a-zA-Z$€£¥₹₩₽₺₦₵₪฿]+(?:\s+[a-zA-Z]+)?)\s+(?:to|in|into|as|=)\s+([a-zA-Z$€£¥₹₩₽₺₦₵₪฿]+(?:\s+[a-zA-Z]+)?)$",
    )
    .expect("valid conversion regex"),
    prefixes: Regex::new(r"(?i)^(?:convert|how much is|what(?:'s|s| is))\s+")
        .expect("valid prefix regex"),
    currency_symbol_prefix: Regex::new(r"^([$€£¥₹₩₽₺₦₵₪฿])\s*([\d.,]+)")
        .expect("valid symbol-prefix regex"),
}
});

pub fn normalize_whitespace(input: &str) -> String {
    let trimmed = input.trim();
    REGEXES.whitespace.replace_all(trimmed, " ").into_owned()
}

pub fn normalize_conversion_input(input: &str) -> String {
    let trimmed = input.trim();
    let mut normalized = REGEXES.whitespace.replace_all(trimmed, " ").into_owned();

    normalized = REGEXES.prefixes.replace(&normalized, "").into_owned();

    REGEXES
        .currency_symbol_prefix
        .replace(&normalized, "$2 $1")
        .into_owned()
}
