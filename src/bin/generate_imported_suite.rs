use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

use async_trait::async_trait;
use smart_calculator::{
    calculate,
    provider::static_rates::{static_crypto_rate, static_fiat_rate},
    types::{Config, RateProvider, ResultType},
};

const MODULE_ORDER: [&str; 7] = ["math", "unit", "time", "date", "currency", "crypto", "mixed"];

#[derive(Debug, Clone)]
struct SourceCase {
    input: String,
    upstream_expected_type: Option<String>,
    description: String,
    section: String,
}

#[derive(Debug, Clone)]
enum ExpectedOutcome {
    Ok(&'static str),
    Err,
}

#[derive(Debug, Clone)]
struct GeneratedCase {
    source: SourceCase,
    expected_outcome: ExpectedOutcome,
}

struct StaticRateProvider;

#[async_trait]
impl RateProvider for StaticRateProvider {
    async fn get_fiat_rate(
        &self,
        base: &str,
        target: &str,
    ) -> std::result::Result<f64, Box<dyn Error>> {
        static_fiat_rate(base.to_string(), target.to_string())
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    async fn get_crypto_rate(
        &self,
        base: &str,
        target: &str,
    ) -> std::result::Result<f64, Box<dyn Error>> {
        static_crypto_rate(base.to_string(), target.to_string())
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    block_on(async_main())
}

async fn async_main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let source_path = manifest_dir.join("SuperCalculator-main/test.js");
    let output_dir = manifest_dir.join("tests");
    let support_dir = output_dir.join("support");

    let source = fs::read_to_string(&source_path)?;
    let cases = parse_cases(&source)?;

    if cases.is_empty() {
        return Err("no test cases found in SuperCalculator-main/test.js".into());
    }

    let mut grouped: BTreeMap<&'static str, Vec<GeneratedCase>> = BTreeMap::new();
    for module in MODULE_ORDER {
        grouped.insert(module, Vec::new());
    }

    for case in cases {
        let module = module_for_section(&case.section);
        let expected_outcome = evaluate_outcome(&case.input).await;
        grouped.entry(module).or_default().push(GeneratedCase {
            source: case,
            expected_outcome,
        });
    }

    fs::create_dir_all(&support_dir)?;
    write_support_mod_rs(&support_dir)?;

    for module in MODULE_ORDER {
        let cases = grouped.remove(module).unwrap_or_default();
        write_module_file(&output_dir.join(format!("{module}.rs")), module, &cases)?;
    }

    println!("Generated Rust test suite in {}", output_dir.display());
    Ok(())
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = noop_waker();
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(&waker);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn build_config() -> Config {
    Config::new()
        .with_rate_provider(Arc::new(StaticRateProvider))
        .with_timezone("UTC")
        .with_locale("en-US")
        .with_precision(10)
}

async fn evaluate_outcome(input: &str) -> ExpectedOutcome {
    match calculate(input, Some(build_config())).await {
        Ok(result) => ExpectedOutcome::Ok(result_type_name(&result.res_type)),
        Err(_) => ExpectedOutcome::Err,
    }
}

fn result_type_name(result_type: &ResultType) -> &'static str {
    match result_type {
        ResultType::Math => "math",
        ResultType::Unit => "unit",
        ResultType::Currency => "currency",
        ResultType::Crypto => "crypto",
        ResultType::Time => "time",
        ResultType::Date => "date",
    }
}

fn write_support_mod_rs(support_dir: &Path) -> Result<(), Box<dyn Error>> {
    let content = r#"use std::{error::Error, sync::Arc};
use std::{
    future::Future,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

use async_trait::async_trait;
use smart_calculator::{
    calculate,
    provider::static_rates::{static_crypto_rate, static_fiat_rate},
    types::{Config, RateProvider, ResultType},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Case {
    pub input: &'static str,
    pub upstream_expected_type: Option<&'static str>,
    pub description: &'static str,
    pub expected_outcome: ExpectedOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpectedOutcome {
    Ok(&'static str),
    Err,
}

pub const fn case(
    input: &'static str,
    upstream_expected_type: Option<&'static str>,
    description: &'static str,
    expected_outcome: ExpectedOutcome,
) -> Case {
    Case {
        input,
        upstream_expected_type,
        description,
        expected_outcome,
    }
}

pub fn case_label(case: &Case) -> String {
    format!("{} ({})", case.input, case.description)
}

struct StaticRateProvider;

#[async_trait]
impl RateProvider for StaticRateProvider {
    async fn get_fiat_rate(
        &self,
        base: &str,
        target: &str,
    ) -> std::result::Result<f64, Box<dyn Error>> {
        static_fiat_rate(base.to_string(), target.to_string())
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    async fn get_crypto_rate(
        &self,
        base: &str,
        target: &str,
    ) -> std::result::Result<f64, Box<dyn Error>> {
        static_crypto_rate(base.to_string(), target.to_string())
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }
}

pub fn build_config() -> Config {
    Config::new()
        .with_rate_provider(Arc::new(StaticRateProvider))
        .with_timezone("UTC")
        .with_locale("en-US")
        .with_precision(10)
}

pub fn block_on<F: Future>(future: F) -> F::Output {
    let waker = noop_waker();
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(&waker);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

pub async fn run_cases(suite_name: &str, cases: Vec<Case>) {
    let mut mismatches = Vec::new();

    for case in cases {
        let actual = match calculate(case.input, Some(build_config())).await {
            Ok(result) => ExpectedOutcome::Ok(result_type_name(result.res_type)),
            Err(_) => ExpectedOutcome::Err,
        };

        if actual != case.expected_outcome {
            mismatches.push(format!(
                "{} | expected {:?}, got {:?}, upstream {:?}",
                case_label(&case),
                case.expected_outcome,
                actual,
                case.upstream_expected_type
            ));
        }
    }

    if !mismatches.is_empty() {
        let mismatch_count = mismatches.len();
        let preview = mismatches
            .into_iter()
            .take(20)
            .collect::<Vec<_>>()
            .join("\n");
        panic!("{suite_name}: {mismatch_count} imported cases mismatched:\n{preview}");
    }
}

pub const fn result_type_name(result_type: ResultType) -> &'static str {
    match result_type {
        ResultType::Math => "math",
        ResultType::Unit => "unit",
        ResultType::Currency => "currency",
        ResultType::Crypto => "crypto",
        ResultType::Time => "time",
        ResultType::Date => "date",
    }
}

fn noop_waker() -> Waker {
    unsafe { Waker::from_raw(noop_raw_waker()) }
}

fn noop_raw_waker() -> RawWaker {
    RawWaker::new(std::ptr::null(), &NOOP_WAKER_VTABLE)
}

static NOOP_WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(clone_noop, wake_noop, wake_by_ref_noop, drop_noop);

unsafe fn clone_noop(_: *const ()) -> RawWaker {
    noop_raw_waker()
}

unsafe fn wake_noop(_: *const ()) {}

unsafe fn wake_by_ref_noop(_: *const ()) {}

unsafe fn drop_noop(_: *const ()) {}
"#;

    fs::write(support_dir.join("mod.rs"), content)?;
    Ok(())
}

fn write_module_file(
    path: &Path,
    module: &str,
    cases: &[GeneratedCase],
) -> Result<(), Box<dyn Error>> {
    let mut lines = Vec::new();
    lines.push("#[path = \"support/mod.rs\"]".to_string());
    lines.push("mod support;".to_string());
    lines.push(String::new());
    lines.push("use support::{Case, ExpectedOutcome, block_on, case, run_cases};".to_string());
    lines.push(String::new());
    lines.push("pub fn cases() -> Vec<Case> {".to_string());
    lines.push("    vec![".to_string());

    let mut current_section: Option<&str> = None;
    for generated in cases {
        let section = generated.source.section.as_str();
        if current_section != Some(section) {
            if !matches!(module, "mixed") || current_section.is_some() {
                lines.push(String::new());
            }
            lines.push(format!("        // {}", section));
            current_section = Some(section);
        }

        let input = rust_string(&generated.source.input);
        let description = rust_string(&generated.source.description);
        let upstream = match &generated.source.upstream_expected_type {
            Some(value) => format!("Some({})", rust_string(value)),
            None => "None".to_string(),
        };
        let expected = match generated.expected_outcome {
            ExpectedOutcome::Ok(kind) => {
                format!("ExpectedOutcome::Ok({})", rust_string(kind))
            }
            ExpectedOutcome::Err => "ExpectedOutcome::Err".to_string(),
        };

        lines.push(format!(
            "        case({}, {}, {}, {}),",
            input, upstream, description, expected
        ));
    }

    lines.push("    ]".to_string());
    lines.push("}".to_string());
    lines.push(String::new());
    lines.push("#[test]".to_string());
    lines.push(format!("fn imported_super_calculator_{module}_cases() {{"));
    lines.push(format!("    block_on(run_cases(\"{module}\", cases()));"));
    lines.push("}".to_string());
    lines.push(String::new());

    fs::write(path, lines.join("\n"))?;
    Ok(())
}

fn rust_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    escaped.push('"');
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }
    escaped.push('"');
    escaped
}

fn parse_cases(source: &str) -> Result<Vec<SourceCase>, Box<dyn Error>> {
    let mut cases = Vec::new();
    let mut current_section = String::new();

    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(section) = parse_section_heading(trimmed) {
            current_section = section.to_string();
            continue;
        }

        if trimmed.starts_with("t(") {
            let mut parser = CallParser::new(trimmed);
            cases.push(SourceCase {
                input: parser.parse_string()?,
                upstream_expected_type: parser.parse_optional_string()?,
                description: parser.parse_string()?,
                section: current_section.clone(),
            });
        }
    }

    Ok(cases)
}

fn parse_section_heading(line: &str) -> Option<&str> {
    let line = line.strip_prefix("// ")?;
    let (_, rest) = line.split_once(". ")?;
    Some(rest.trim())
}

fn module_for_section(section: &str) -> &'static str {
    if section.starts_with("MATH") {
        "math"
    } else if section.starts_with("UNIT CONVERSIONS")
        || section.starts_with("UNIT CONVERSION")
    {
        "unit"
    } else if section.starts_with("TIME ZONES") {
        "time"
    } else if section.starts_with("DATES") {
        "date"
    } else if section.starts_with("CURRENCY") {
        "currency"
    } else if section.starts_with("CRYPTO") {
        "crypto"
    } else {
        "mixed"
    }
}

struct CallParser {
    chars: Vec<char>,
    pos: usize,
}

impl CallParser {
    fn new(line: &str) -> Self {
        Self {
            chars: line.chars().collect(),
            pos: 2,
        }
    }

    fn parse_string(&mut self) -> Result<String, Box<dyn Error>> {
        self.skip_ws();
        let quote = self.next_char().ok_or("unexpected end of line")?;
        if quote != '\'' && quote != '"' {
            return Err(format!("expected string literal, found '{quote}'").into());
        }

        let mut value = String::new();
        while let Some(ch) = self.next_char() {
            if ch == '\\' {
                let escaped = self.next_char().ok_or("unterminated escape sequence")?;
                value.push(match escaped {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\'' => '\'',
                    '"' => '"',
                    '\\' => '\\',
                    other => other,
                });
                continue;
            }

            if ch == quote {
                self.skip_ws();
                if self.peek_char() == Some(',') {
                    self.pos += 1;
                }
                return Ok(value);
            }

            value.push(ch);
        }

        Err("unterminated string literal".into())
    }

    fn parse_optional_string(&mut self) -> Result<Option<String>, Box<dyn Error>> {
        self.skip_ws();
        if self.starts_with("null") {
            self.pos += 4;
            self.skip_ws();
            if self.peek_char() == Some(',') {
                self.pos += 1;
            }
            return Ok(None);
        }

        self.parse_string().map(Some)
    }

    fn starts_with(&self, needle: &str) -> bool {
        let end = self.pos + needle.chars().count();
        self.chars
            .get(self.pos..end)
            .map(|slice| slice.iter().collect::<String>() == needle)
            .unwrap_or(false)
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek_char(), Some(ch) if ch.is_whitespace()) {
            self.pos += 1;
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += 1;
        Some(ch)
    }
}

fn noop_waker() -> Waker {
    unsafe { Waker::from_raw(noop_raw_waker()) }
}

fn noop_raw_waker() -> RawWaker {
    RawWaker::new(std::ptr::null(), &NOOP_WAKER_VTABLE)
}

static NOOP_WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(clone_noop, wake_noop, wake_by_ref_noop, drop_noop);

unsafe fn clone_noop(_: *const ()) -> RawWaker {
    noop_raw_waker()
}

unsafe fn wake_noop(_: *const ()) {}

unsafe fn wake_by_ref_noop(_: *const ()) {}

unsafe fn drop_noop(_: *const ()) {}
