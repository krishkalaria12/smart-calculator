# Smart Calculator

Natural-language calculation for Rust.

`smart-calculator` takes a single string like `sqrt(144)`, `3 km to m`, `time in tokyo`, or `100 usd to inr` and returns a structured result with the detected intent, raw value, formatted output, and optional metadata.

## What It Does

| Capability | Examples |
| --- | --- |
| Math | `2^10`, `5!`, `0xFF`, `square root of 144`, `20% of 500` |
| Unit conversion | `3 km to m`, `100 fahrenheit to celsius`, `1 acre to m²` |
| Time zones | `time in tokyo`, `pst to est`, `12:30 ist to london` |
| Date queries | `tomorrow`, `next friday`, `1741000000`, `today to unix` |
| Fiat currency | `usd to eur`, `100 dollars to rupees` |
| Crypto | `btc to usd`, `100 usd to eth`, `bitcoin to inr` |

## Highlights

- Single async entrypoint: `calculate(input, options)`
- Natural-language parsing with intent detection
- Structured Rust types for results and configuration
- Locale-aware number formatting
- Online-first rate providers with static fallbacks
- No API keys required for built-in currency and crypto lookups
- Backed by 700+ imported compatibility tests

## Quick Start

### 1. Add the crate locally

If you are working in a workspace or from source, add it as a path dependency:

```toml
[dependencies]
smart-calculator = { path = "../smart-calculator" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### 2. Call `calculate`

```rust
use smart_calculator::calculate;

#[tokio::main]
async fn main() {
    let result = calculate("3 km to m", None).await.unwrap();

    println!("{:?}", result.res_type);
    println!("{}", result.formatted);
}
```

### Example outputs

```rust
use smart_calculator::calculate;

# #[tokio::main]
# async fn main() {
let math = calculate("sqrt(144)", None).await.unwrap();
let unit = calculate("5 kg to lb", None).await.unwrap();
let time = calculate("time in tokyo", None).await.unwrap();
let date = calculate("next friday", None).await.unwrap();
let fiat = calculate("100 usd to eur", None).await.unwrap();
let crypto = calculate("0.5 btc to usd", None).await.unwrap();

assert_eq!(math.formatted, "12");
println!("{}", unit.formatted);
println!("{}", time.formatted);
println!("{}", date.formatted);
println!("{}", fiat.formatted);
println!("{}", crypto.formatted);
# }
```

## API

### `calculate`

```rust
pub async fn calculate(input: &str, options: Option<Config>) -> Result<CalculatorResult>
```

### `Config`

```rust
use smart_calculator::types::Config;

let config = Config::new()
    .with_timezone("Asia/Kolkata")
    .with_locale("en-US")
    .with_precision(10);
```

Available options:

- `with_rate_provider(...)` to override fiat/crypto pricing
- `with_timezone(...)` to control local time assumptions
- `with_locale(...)` to control formatting
- `with_precision(...)` to clamp significant digits from `1..=21`

### `CalculatorResult`

```rust
pub struct CalculatorResult {
    pub res_type: ResultType,
    pub input: String,
    pub result: AnswerType,
    pub formatted: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}
```

`ResultType` can be:

- `Math`
- `Unit`
- `Currency`
- `Crypto`
- `Time`
- `Date`

## Supported Inputs

### Math

- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Exponentiation: `^`, `**`
- Bitwise: `&`, `|`, `~`, `<<`, `>>`
- Constants: `pi`, `e`, `tau`, `phi`, `infinity`
- Functions: `sqrt`, `cbrt`, `abs`, `ceil`, `floor`, `round`, `log`, `log2`, `log10`, `ln`, `exp`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `sinh`, `cosh`, `tanh`, `pow`, `max`, `min`
- Literals: hex, binary, octal, scientific notation
- Natural language: `what is 5 + 3`, `factorial of 5`, `2 to the power of 10`

### Units

Supports conversions across:

- Length
- Mass
- Volume
- Area
- Temperature
- Speed
- Time
- Data
- Pressure
- Energy
- Power
- Angle
- Frequency
- Electric current
- Voltage
- Fuel economy

Examples:

- `1 light year to km`
- `100 celsius to fahrenheit`
- `1 gb to byte`
- `360 deg to revolution`
- `30 mpg to km/l`

### Time and Dates

- `time in london`
- `utc to ist`
- `midnight utc to pst`
- `today`
- `day after tomorrow`
- `2 weeks ago`
- `2025-03-03`
- `now to unix`
- `from 2025-01-01 to 2025-12-31`

### Fiat and Crypto

- `usd to inr`
- `100 dollars to euros`
- `btc to usd`
- `100 usd to bitcoin`
- `ethereum to rupees`

Built-in providers use multiple public endpoints and fall back to static reference rates when network providers fail.

## Custom Rate Providers

You can inject your own provider by implementing `RateProvider`.

```rust
use async_trait::async_trait;
use smart_calculator::types::RateProvider;
use std::error::Error;

struct MyProvider;

#[async_trait]
impl RateProvider for MyProvider {
    async fn get_fiat_rate(
        &self,
        base: &str,
        target: &str,
    ) -> Result<f64, Box<dyn Error>> {
        let _ = (base, target);
        Ok(1.0)
    }

    async fn get_crypto_rate(
        &self,
        base: &str,
        target: &str,
    ) -> Result<f64, Box<dyn Error>> {
        let _ = (base, target);
        Ok(1.0)
    }
}
```

## Project Shape

| Path | Purpose |
| --- | --- |
| `src/lib.rs` | public entrypoint |
| `src/parser/` | intent detection and normalization |
| `src/evaluators/` | math, unit, time, date, currency, crypto evaluation |
| `src/provider/` | live and static rate providers |
| `src/data/` | unit, currency, timezone, and math data |
| `tests/` | imported compatibility suites by category |

## Development

Run the full test suite:

```bash
cargo test
```

Format the codebase:

```bash
cargo fmt
```

## Notes

- This repository is library-first. The binary target is currently a placeholder and not a CLI interface.
- Empty input returns an error.
- Currency and crypto conversions are async because they may hit network providers.
- Formatting is locale-aware, but parsing is driven by the normalized input language rules in the parser.

## License

MIT
