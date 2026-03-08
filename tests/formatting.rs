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
    types::{Config, RateProvider},
};

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

fn build_config() -> Config {
    Config::new()
        .with_rate_provider(Arc::new(StaticRateProvider))
        .with_timezone("UTC")
        .with_locale("en-US")
        .with_precision(10)
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

#[test]
fn unit_output_trims_trailing_zero_decimals() {
    let result = block_on(calculate("1 km to m", Some(build_config()))).unwrap();

    assert_eq!(result.formatted, "1,000 meter");
}

#[test]
fn math_output_caps_visible_decimals() {
    let result = block_on(calculate("1 / 3", Some(build_config()))).unwrap();

    assert_eq!(result.formatted, "0.333");
}

#[test]
fn fiat_output_keeps_needed_decimals_only() {
    let result = block_on(calculate("1 usd to eur", Some(build_config()))).unwrap();

    assert_eq!(result.formatted, "€0.92");
}

#[test]
fn crypto_output_caps_visible_decimals() {
    let result = block_on(calculate("100 usd to btc", Some(build_config()))).unwrap();

    assert_eq!(result.formatted, "0.001 BTC");
}
