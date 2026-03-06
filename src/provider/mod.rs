pub mod error;
pub mod static_rates;

mod crypto;
mod fiat;
mod models;

use async_trait::async_trait;
use std::error::Error;

use crate::types::RateProvider;

pub use crypto::{coincap_rate, get_crypto_rate_with_fiat_fallback};
pub use fiat::get_fiat_rate;

pub struct DefaultProvider;

#[async_trait]
impl RateProvider for DefaultProvider {
    async fn get_fiat_rate(
        &self,
        base: &str,
        target: &str,
    ) -> std::result::Result<f64, Box<dyn Error>> {
        get_fiat_rate(base, target)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    async fn get_crypto_rate(
        &self,
        base: &str,
        target: &str,
    ) -> std::result::Result<f64, Box<dyn Error>> {
        get_crypto_rate_with_fiat_fallback(base, target, |b, t| async move {
            get_fiat_rate(&b, &t)
                .await
                .map_err(|e| Box::new(e) as Box<dyn Error>)
        })
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}
