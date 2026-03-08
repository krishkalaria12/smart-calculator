use fixed_decimal::{Decimal as FixedDecimal, FloatPrecision};
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::data::math::{CONSTANTS, FUNCTIONS, FUNCTIONS2, Number};
use crate::evaluators::error::{EvaluatorError, Result};
use crate::formatting::{MAX_DISPLAY_FRACTION_DIGITS, format_display_number, normalize_locale};
use crate::parser::REGEXES;
use crate::types::{AnswerType, CalculatorResult, ResultType};

const DEFAULT_PRECISION: u8 = 10;

pub fn evaluate_math(
    expression: String,
    locale: Option<String>,
    precision: Option<u8>,
) -> Result<CalculatorResult> {
    let normalized_expression = normalize_math_expression(&expression);
    let precision = normalize_precision(precision);
    let locale = normalize_locale(locale);

    let mut parser = MathParser::new(&normalized_expression);
    let result = parser.parse()?;
    let rounded = round_to_significant_digits(result, precision);
    let formatted = format_math_value(rounded, &locale, precision);
    let metadata = build_math_metadata(rounded, &normalized_expression);

    Ok(CalculatorResult {
        res_type: ResultType::Math,
        input: expression,
        result: AnswerType::Number(rounded),
        formatted,
        metadata: Some(metadata),
    })
}

struct MathParser<'a> {
    expr: &'a str,
    chars: Vec<char>,
    pos: usize,
}

impl<'a> MathParser<'a> {
    fn new(expression: &'a str) -> Self {
        Self {
            expr: expression,
            chars: expression.chars().collect(),
            pos: 0,
        }
    }

    fn parse(&mut self) -> Result<Number> {
        let result = self.parse_bitwise_or()?;
        self.skip_whitespace();

        if let Some(ch) = self.peek_raw() {
            return Err(self.err(format!(
                "unexpected character '{ch}' at position {}",
                self.pos
            )));
        }

        Ok(result)
    }

    fn parse_bitwise_or(&mut self) -> Result<Number> {
        let mut left = self.parse_bitwise_and()?;

        while self.peek() == Some('|') {
            self.consume(None)?;
            let right = self.parse_bitwise_and()?;
            left = (to_bitwise_int(left)? | to_bitwise_int(right)?) as Number;
        }

        Ok(left)
    }

    fn parse_bitwise_and(&mut self) -> Result<Number> {
        let mut left = self.parse_bitwise_shift()?;

        while self.peek() == Some('&') {
            self.consume(None)?;
            let right = self.parse_bitwise_shift()?;
            left = (to_bitwise_int(left)? & to_bitwise_int(right)?) as Number;
        }

        Ok(left)
    }

    fn parse_bitwise_shift(&mut self) -> Result<Number> {
        let mut left = self.parse_addition()?;

        loop {
            self.skip_whitespace();

            if self.match_pair('<', '<') {
                self.pos += 2;
                let right = self.parse_addition()?;
                let shift = to_shift_amount(right)?;
                left = (to_bitwise_int(left)? << shift) as Number;
                continue;
            }

            if self.match_pair('>', '>') {
                self.pos += 2;
                let right = self.parse_addition()?;
                let shift = to_shift_amount(right)?;
                left = (to_bitwise_int(left)? >> shift) as Number;
                continue;
            }

            break;
        }

        Ok(left)
    }

    fn parse_addition(&mut self) -> Result<Number> {
        let mut left = self.parse_multiplication()?;

        loop {
            match self.peek() {
                Some('+') => {
                    self.consume(Some('+'))?;
                    left += self.parse_multiplication()?;
                }
                Some('-') => {
                    self.consume(Some('-'))?;
                    left -= self.parse_multiplication()?;
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Number> {
        let mut left = self.parse_exponentiation()?;

        loop {
            match self.peek() {
                Some('*') if !self.match_pair('*', '*') => {
                    self.consume(Some('*'))?;
                    left *= self.parse_exponentiation()?;
                }
                Some('/') => {
                    self.consume(Some('/'))?;
                    let right = self.parse_exponentiation()?;
                    if right == 0.0 {
                        return Err(self.err("division by zero"));
                    }
                    left /= right;
                }
                Some('%') if !self.should_treat_as_percentage() => {
                    self.consume(Some('%'))?;
                    let right = self.parse_exponentiation()?;
                    if right == 0.0 {
                        return Err(self.err("modulo by zero"));
                    }
                    left %= right;
                }
                Some(ch) if starts_implicit_multiplication(ch) => {
                    left *= self.parse_exponentiation()?;
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_exponentiation(&mut self) -> Result<Number> {
        let base = self.parse_unary()?;

        if self.peek() == Some('^') {
            self.consume(Some('^'))?;
            let exponent = self.parse_exponentiation()?;
            return Ok(base.powf(exponent));
        }

        if self.peek() == Some('*') && self.match_pair('*', '*') {
            self.pos += 2;
            let exponent = self.parse_exponentiation()?;
            return Ok(base.powf(exponent));
        }

        Ok(base)
    }

    fn parse_unary(&mut self) -> Result<Number> {
        match self.peek() {
            Some('-') => {
                self.consume(Some('-'))?;
                Ok(-self.parse_unary()?)
            }
            Some('+') => {
                self.consume(Some('+'))?;
                self.parse_unary()
            }
            Some('~') => {
                self.consume(Some('~'))?;
                Ok((!to_bitwise_int(self.parse_unary()?)?) as Number)
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Number> {
        let mut value = self.parse_primary()?;

        loop {
            self.skip_whitespace();

            match self.peek_raw() {
                Some('!') => {
                    self.pos += 1;
                    value = factorial(value)?;
                }
                Some('%') if self.should_treat_as_percentage() => {
                    self.pos += 1;
                    value /= 100.0;
                }
                _ => break,
            }
        }

        Ok(value)
    }

    fn parse_primary(&mut self) -> Result<Number> {
        self.skip_whitespace();

        match self.peek_raw() {
            Some('(') => {
                self.consume(Some('('))?;
                let value = self.parse_bitwise_or()?;
                self.consume(Some(')'))?;
                Ok(value)
            }
            Some(ch) if ch.is_ascii_digit() || ch == '.' => self.parse_number(),
            Some(ch) if is_identifier_start(ch) => self.parse_identifier(),
            Some(ch) => Err(self.err(format!(
                "unexpected character '{ch}' at position {}",
                self.pos
            ))),
            None => Err(self.err("unexpected end of expression")),
        }
    }

    fn parse_number(&mut self) -> Result<Number> {
        let start = self.pos;

        if self.peek_raw() == Some('0') {
            if matches!(self.peek_offset(1), Some('x' | 'X')) {
                self.pos += 2;
                while matches!(self.peek_raw(), Some(ch) if ch.is_ascii_hexdigit()) {
                    self.pos += 1;
                }
                let raw = self.slice(start + 2, self.pos);
                if raw.is_empty() {
                    return Err(self.err("invalid hexadecimal literal"));
                }
                return i64::from_str_radix(raw, 16)
                    .map(|value| value as Number)
                    .map_err(|_| self.err(format!("invalid hexadecimal literal '{raw}'")));
            }

            if matches!(self.peek_offset(1), Some('b' | 'B')) {
                self.pos += 2;
                while matches!(self.peek_raw(), Some('0' | '1')) {
                    self.pos += 1;
                }
                let raw = self.slice(start + 2, self.pos);
                if raw.is_empty() {
                    return Err(self.err("invalid binary literal"));
                }
                return i64::from_str_radix(raw, 2)
                    .map(|value| value as Number)
                    .map_err(|_| self.err(format!("invalid binary literal '{raw}'")));
            }

            if matches!(self.peek_offset(1), Some('o' | 'O')) {
                self.pos += 2;
                while matches!(self.peek_raw(), Some(ch) if ('0'..='7').contains(&ch)) {
                    self.pos += 1;
                }
                let raw = self.slice(start + 2, self.pos);
                if raw.is_empty() {
                    return Err(self.err("invalid octal literal"));
                }
                return i64::from_str_radix(raw, 8)
                    .map(|value| value as Number)
                    .map_err(|_| self.err(format!("invalid octal literal '{raw}'")));
            }
        }

        while matches!(self.peek_raw(), Some(ch) if ch.is_ascii_digit()) {
            self.pos += 1;
        }

        if self.peek_raw() == Some('.') {
            self.pos += 1;
            while matches!(self.peek_raw(), Some(ch) if ch.is_ascii_digit()) {
                self.pos += 1;
            }
        }

        if matches!(self.peek_raw(), Some('e' | 'E')) {
            let exponent_start = self.pos;
            self.pos += 1;

            if matches!(self.peek_raw(), Some('+' | '-')) {
                self.pos += 1;
            }

            let digit_start = self.pos;
            while matches!(self.peek_raw(), Some(ch) if ch.is_ascii_digit()) {
                self.pos += 1;
            }

            if digit_start == self.pos {
                self.pos = exponent_start;
            }
        }

        let raw = self.slice(start, self.pos);
        raw.parse::<Number>()
            .map_err(|_| self.err(format!("invalid number '{raw}'")))
    }

    fn parse_identifier(&mut self) -> Result<Number> {
        let start = self.pos;

        while matches!(self.peek_raw(), Some(ch) if is_identifier_part(ch)) {
            self.pos += 1;
        }

        let name = self.slice(start, self.pos).to_lowercase();
        self.skip_whitespace();

        if self.peek_raw() == Some('(') {
            if name == "max" || name == "min" {
                let args = self.parse_arguments()?;
                if args.is_empty() {
                    return Err(
                        self.err(format!("function '{name}' requires at least one argument"))
                    );
                }

                let value = if name == "max" {
                    args.into_iter().fold(Number::NEG_INFINITY, Number::max)
                } else {
                    args.into_iter().fold(Number::INFINITY, Number::min)
                };
                return Ok(value);
            }

            if let Some(function) = FUNCTIONS2.get(name.as_str()) {
                let args = self.parse_arguments()?;
                if args.len() != 2 {
                    return Err(self.err(format!("function '{name}' expects 2 arguments")));
                }
                return Ok(function(args[0], args[1]));
            }

            if let Some(function) = FUNCTIONS.get(name.as_str()) {
                self.consume(Some('('))?;
                let arg = self.parse_bitwise_or()?;
                self.consume(Some(')'))?;
                return Ok(function(arg));
            }

            return Err(self.err(format!("unknown function '{name}'")));
        }

        if let Some(value) = CONSTANTS.get(name.as_str()) {
            return Ok(*value);
        }

        Err(self.err(format!("unknown identifier '{name}'")))
    }

    fn parse_arguments(&mut self) -> Result<Vec<Number>> {
        let mut args = Vec::new();

        self.consume(Some('('))?;
        self.skip_whitespace();

        if self.peek_raw() == Some(')') {
            self.consume(Some(')'))?;
            return Ok(args);
        }

        loop {
            args.push(self.parse_bitwise_or()?);
            self.skip_whitespace();

            if self.peek_raw() == Some(',') {
                self.pos += 1;
                continue;
            }

            break;
        }

        self.consume(Some(')'))?;
        Ok(args)
    }

    fn should_treat_as_percentage(&self) -> bool {
        let mut i = self.pos + 1;
        while let Some(ch) = self.chars.get(i) {
            if ch.is_whitespace() {
                i += 1;
            } else {
                break;
            }
        }

        match self.chars.get(i) {
            None => true,
            Some(ch) => matches!(
                ch,
                '+' | '-' | '*' | '/' | '^' | ')' | '%' | '|' | '&' | '<' | '>'
            ),
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek_raw(), Some(ch) if ch.is_whitespace()) {
            self.pos += 1;
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.skip_whitespace();
        self.peek_raw()
    }

    fn peek_raw(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_offset(&self, offset: usize) -> Option<char> {
        self.chars.get(self.pos + offset).copied()
    }

    fn consume(&mut self, expected: Option<char>) -> Result<char> {
        self.skip_whitespace();
        let ch = self
            .peek_raw()
            .ok_or_else(|| self.err("unexpected end of expression"))?;

        if let Some(expected) = expected {
            if ch != expected {
                return Err(self.err(format!(
                    "expected '{expected}' at position {}, got '{ch}'",
                    self.pos
                )));
            }
        }

        self.pos += 1;
        Ok(ch)
    }

    fn match_pair(&self, first: char, second: char) -> bool {
        self.peek_raw() == Some(first) && self.peek_offset(1) == Some(second)
    }

    fn slice(&self, start: usize, end: usize) -> &str {
        let mut iter = self.expr.char_indices();
        let start_byte = iter
            .nth(start)
            .map(|(idx, _)| idx)
            .unwrap_or(self.expr.len());
        let end_byte = self
            .expr
            .char_indices()
            .nth(end)
            .map(|(idx, _)| idx)
            .unwrap_or(self.expr.len());
        &self.expr[start_byte..end_byte]
    }

    fn err(&self, message: impl Into<String>) -> EvaluatorError {
        EvaluatorError::MathEvaluationFailed(message.into())
    }
}

fn factorial(value: Number) -> Result<Number> {
    if value < 0.0 {
        return Err(EvaluatorError::MathEvaluationFailed(
            "factorial of negative number".to_string(),
        ));
    }

    if value.fract() != 0.0 {
        return Err(EvaluatorError::MathEvaluationFailed(
            "factorial of non-integer".to_string(),
        ));
    }

    if value > 170.0 {
        return Ok(Number::INFINITY);
    }

    let n = value as u32;
    let mut result = 1.0;
    for i in 2..=n {
        result *= i as Number;
    }

    Ok(result)
}

fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || matches!(ch, '_' | 'π' | 'τ' | 'φ')
}

fn is_identifier_part(ch: char) -> bool {
    is_identifier_start(ch) || ch.is_ascii_digit()
}

fn starts_implicit_multiplication(ch: char) -> bool {
    matches!(ch, '(' | 'π' | 'τ' | 'φ' | '_') || ch.is_ascii_alphabetic()
}

fn to_bitwise_int(value: Number) -> Result<i64> {
    if !value.is_finite() {
        return Err(EvaluatorError::MathEvaluationFailed(
            "bitwise operations require finite numbers".to_string(),
        ));
    }

    Ok(value.trunc() as i64)
}

fn to_shift_amount(value: Number) -> Result<u32> {
    if !value.is_finite() || value < 0.0 || value.fract() != 0.0 {
        return Err(EvaluatorError::MathEvaluationFailed(
            "shift amount must be a non-negative integer".to_string(),
        ));
    }

    Ok((value as u32).min(63))
}

fn normalize_math_expression(expression: &str) -> String {
    let mut normalized = expression
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    normalized = REGEXES
        .math_prefix_cleanup
        .replace(&normalized, "")
        .to_string();
    normalized = REGEXES
        .math_the_cleanup
        .replace(&normalized, "")
        .to_string();

    for (pattern, replacement) in &REGEXES.math_wrappers {
        if pattern.is_match(&normalized) {
            normalized = pattern.replace(&normalized, *replacement).to_string();
            break;
        }
    }

    for (pattern, replacement) in &REGEXES.math_word_operators {
        normalized = pattern.replace_all(&normalized, *replacement).to_string();
    }

    normalized.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalize_precision(precision: Option<u8>) -> u8 {
    precision.unwrap_or(DEFAULT_PRECISION).clamp(1, 21)
}

fn round_to_significant_digits(value: Number, precision: u8) -> Number {
    if !value.is_finite() || value == 0.0 {
        return value;
    }

    FixedDecimal::try_from_f64(value, FloatPrecision::SignificantDigits(precision))
        .ok()
        .and_then(|decimal| decimal.to_string().parse::<Number>().ok())
        .unwrap_or(value)
}

fn format_math_value(value: Number, locale_str: &str, precision: u8) -> String {
    if !value.is_finite() {
        return value.to_string();
    }

    format_display_number(
        value,
        locale_str,
        precision.min(MAX_DISPLAY_FRACTION_DIGITS),
    )
}

fn build_math_metadata(result: Number, normalized_expression: &str) -> HashMap<String, Value> {
    let mut metadata = HashMap::from([(
        "normalizedExpression".to_string(),
        json!(normalized_expression),
    )]);

    if result.is_finite() && result.fract() == 0.0 {
        let integer = result.trunc() as i64;
        metadata.insert("hex".to_string(), json!(format!("0x{:X}", integer)));
        metadata.insert("binary".to_string(), json!(format!("0b{:b}", integer)));
        metadata.insert("octal".to_string(), json!(format!("0o{:o}", integer)));
    }

    metadata
}
