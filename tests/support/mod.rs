use std::{
    error::Error,
    future::Future,
    sync::Arc,
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
            Ok(result) => Ok(result_type_name(result.res_type)),
            Err(_) => Err(()),
        };

        if !matches_upstream_expectation(case.upstream_expected_type, &actual) {
            mismatches.push(format!(
                "{} | expected upstream {:?}, got {:?}, previous snapshot {:?}",
                case_label(&case),
                case.upstream_expected_type,
                actual,
                case.expected_outcome
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

fn matches_upstream_expectation(expected: Option<&str>, actual: &Result<&'static str, ()>) -> bool {
    match (expected, actual) {
        (Some("error"), Err(())) => true,
        (Some(expected_type), Ok(actual_type)) => expected_type == *actual_type,
        (Some(_), Err(())) => false,
        (None, Ok(_)) => true,
        (None, Err(())) => false,
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
