#[path = "support/mod.rs"]
mod support;

use support::{Case, ExpectedOutcome, block_on, case, run_cases};

pub fn cases() -> Vec<Case> {
    vec![
        // MIXED / AMBIGUITY TESTS
        case("1 m to ft", Some("unit"), "ambiguous m = meter not minute", ExpectedOutcome::Ok("unit")),
        case("1 min to s", Some("unit"), "min = minute", ExpectedOutcome::Ok("unit")),
        case("100 c to f", Some("unit"), "c = celsius shorthand", ExpectedOutcome::Ok("unit")),
        case("pi + e", Some("math"), "constants in expression", ExpectedOutcome::Ok("math")),
        case("2pi", Some("math"), "implicit multiplication with constant", ExpectedOutcome::Ok("math")),
        case("time", None, "bare word \"time\"", ExpectedOutcome::Ok("time")),
        case("date", None, "bare word \"date\"", ExpectedOutcome::Ok("date")),
        case("now", None, "bare word \"now\"", ExpectedOutcome::Ok("date")),

        // CASING & WHITESPACE VARIATIONS
        case("USD TO INR", Some("currency"), "uppercase currency", ExpectedOutcome::Ok("currency")),
        case("Usd To Inr", Some("currency"), "title case currency", ExpectedOutcome::Ok("currency")),
        case("usd  to  inr", Some("currency"), "extra spaces", ExpectedOutcome::Ok("currency")),
        case(" usd to inr ", Some("currency"), "leading/trailing spaces", ExpectedOutcome::Ok("currency")),
        case("TIME IN TOKYO", Some("time"), "uppercase time query", ExpectedOutcome::Ok("time")),
        case("Time In Tokyo", Some("time"), "title case time query", ExpectedOutcome::Ok("time")),
        case("SQRT(144)", Some("math"), "uppercase function", ExpectedOutcome::Ok("math")),
        case("Sqrt(144)", Some("math"), "title case function", ExpectedOutcome::Ok("math")),
        case("TOMORROW", Some("date"), "uppercase date", ExpectedOutcome::Ok("date")),
        case("Tomorrow", Some("date"), "title case date", ExpectedOutcome::Ok("date")),
        case("BTC TO USD", Some("crypto"), "uppercase crypto", ExpectedOutcome::Ok("crypto")),
        case("Bitcoin to USD", Some("crypto"), "mixed case crypto", ExpectedOutcome::Ok("crypto")),
        case("3 KM TO M", Some("unit"), "uppercase units", ExpectedOutcome::Ok("unit")),
    ]
}

#[test]
fn imported_super_calculator_mixed_cases() {
    block_on(run_cases("mixed", cases()));
}
