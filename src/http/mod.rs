pub mod error;

use self::error::{HttpError, Result};
use reqwest::Client;
use serde_json::Value;
use std::sync::LazyLock;
use std::time::Duration;

pub static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .build()
        .expect("Failed to initialize HTTP client")
});

pub async fn fetch_json(url: &str, timeout_ms: Option<u64>) -> Result<Value> {
    let timeout = Duration::from_millis(timeout_ms.unwrap_or(8000));

    let response = HTTP_CLIENT
        .get(url)
        .timeout(timeout)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                HttpError::RequestTimeoutError(e.to_string())
            } else {
                HttpError::HttpRequestError(e.to_string())
            }
        })?;

    let status = response.status();
    if !status.is_success() {
        return Err(HttpError::HttpResponseStatusError {
            url: url.to_string(),
            status: status.as_u16(),
            status_text: status
                .canonical_reason()
                .unwrap_or("Unknown HTTP status")
                .to_string(),
        });
    }

    response
        .json::<Value>()
        .await
        .map_err(|e| HttpError::HttpResponseDecodeError(e.to_string()))
}
