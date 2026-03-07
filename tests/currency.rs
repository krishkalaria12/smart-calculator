#[path = "support/mod.rs"]
mod support;

use support::{Case, ExpectedOutcome, block_on, case, run_cases};

pub fn cases() -> Vec<Case> {
    vec![

        // CURRENCY - Major Pairs
        case("usd to inr", Some("currency"), "USD to INR rate", ExpectedOutcome::Ok("currency")),
        case("100 usd to inr", Some("currency"), "100 USD to INR", ExpectedOutcome::Ok("currency")),
        case("1 usd to inr", Some("currency"), "1 USD to INR", ExpectedOutcome::Ok("currency")),
        case("50 usd to inr", Some("currency"), "50 USD to INR", ExpectedOutcome::Ok("currency")),
        case("1000 usd to inr", Some("currency"), "1000 USD to INR", ExpectedOutcome::Ok("currency")),
        case("usd to eur", Some("currency"), "USD to EUR", ExpectedOutcome::Ok("currency")),
        case("100 usd to eur", Some("currency"), "100 USD to EUR", ExpectedOutcome::Ok("currency")),
        case("usd to gbp", Some("currency"), "USD to GBP", ExpectedOutcome::Ok("currency")),
        case("100 usd to gbp", Some("currency"), "100 USD to GBP", ExpectedOutcome::Ok("currency")),
        case("500 jpy to usd", Some("currency"), "500 JPY to USD", ExpectedOutcome::Ok("currency")),
        case("1000 jpy to usd", Some("currency"), "1000 JPY to USD", ExpectedOutcome::Ok("currency")),
        case("eur to usd", Some("currency"), "EUR to USD", ExpectedOutcome::Ok("currency")),
        case("100 eur to usd", Some("currency"), "100 EUR to USD", ExpectedOutcome::Ok("currency")),
        case("gbp to usd", Some("currency"), "GBP to USD", ExpectedOutcome::Ok("currency")),
        case("100 gbp to usd", Some("currency"), "100 GBP to USD", ExpectedOutcome::Ok("currency")),
        case("eur to gbp", Some("currency"), "EUR to GBP", ExpectedOutcome::Ok("currency")),
        case("gbp to eur", Some("currency"), "GBP to EUR", ExpectedOutcome::Ok("currency")),
        case("inr to usd", Some("currency"), "INR to USD", ExpectedOutcome::Ok("currency")),
        case("1000 inr to usd", Some("currency"), "1000 INR to USD", ExpectedOutcome::Ok("currency")),
        case("10000 inr to usd", Some("currency"), "10000 INR to USD", ExpectedOutcome::Ok("currency")),
        case("cny to usd", Some("currency"), "CNY to USD", ExpectedOutcome::Ok("currency")),
        case("100 cny to usd", Some("currency"), "100 CNY to USD", ExpectedOutcome::Ok("currency")),
        case("aud to usd", Some("currency"), "AUD to USD", ExpectedOutcome::Ok("currency")),
        case("cad to usd", Some("currency"), "CAD to USD", ExpectedOutcome::Ok("currency")),
        case("chf to usd", Some("currency"), "CHF to USD", ExpectedOutcome::Ok("currency")),
        case("krw to usd", Some("currency"), "KRW to USD", ExpectedOutcome::Ok("currency")),
        case("sgd to usd", Some("currency"), "SGD to USD", ExpectedOutcome::Ok("currency")),
        case("hkd to usd", Some("currency"), "HKD to USD", ExpectedOutcome::Ok("currency")),
        case("nzd to usd", Some("currency"), "NZD to USD", ExpectedOutcome::Ok("currency")),
        case("sek to usd", Some("currency"), "SEK to USD", ExpectedOutcome::Ok("currency")),
        case("nok to usd", Some("currency"), "NOK to USD", ExpectedOutcome::Ok("currency")),
        case("dkk to usd", Some("currency"), "DKK to USD", ExpectedOutcome::Ok("currency")),
        case("mxn to usd", Some("currency"), "MXN to USD", ExpectedOutcome::Ok("currency")),
        case("brl to usd", Some("currency"), "BRL to USD", ExpectedOutcome::Ok("currency")),
        case("thb to usd", Some("currency"), "THB to USD", ExpectedOutcome::Ok("currency")),
        case("php to usd", Some("currency"), "PHP to USD", ExpectedOutcome::Ok("currency")),
        case("idr to usd", Some("currency"), "IDR to USD", ExpectedOutcome::Ok("currency")),
        case("myr to usd", Some("currency"), "MYR to USD", ExpectedOutcome::Ok("currency")),
        case("zar to usd", Some("currency"), "ZAR to USD", ExpectedOutcome::Ok("currency")),
        case("aed to usd", Some("currency"), "AED to USD", ExpectedOutcome::Ok("currency")),
        case("sar to usd", Some("currency"), "SAR to USD", ExpectedOutcome::Ok("currency")),
        case("try to usd", Some("currency"), "TRY to USD", ExpectedOutcome::Ok("currency")),
        case("pln to usd", Some("currency"), "PLN to USD", ExpectedOutcome::Ok("currency")),
        case("czk to usd", Some("currency"), "CZK to USD", ExpectedOutcome::Ok("currency")),
        case("rub to usd", Some("currency"), "RUB to USD", ExpectedOutcome::Ok("currency")),

        // CURRENCY - Common Names
        case("dollar to rupee", Some("currency"), "natural: dollar to rupee", ExpectedOutcome::Ok("currency")),
        case("100 dollars to rupees", Some("currency"), "natural: dollars to rupees", ExpectedOutcome::Ok("currency")),
        case("dollar to euro", Some("currency"), "natural: dollar to euro", ExpectedOutcome::Ok("currency")),
        case("100 dollars to euros", Some("currency"), "natural: dollars to euros", ExpectedOutcome::Ok("currency")),
        case("pound to dollar", Some("currency"), "natural: pound to dollar", ExpectedOutcome::Ok("currency")),
        case("100 pounds to dollars", Some("currency"), "natural: pounds to dollars", ExpectedOutcome::Ok("currency")),
        case("euro to dollar", Some("currency"), "natural: euro to dollar", ExpectedOutcome::Ok("currency")),
        case("yen to dollar", Some("currency"), "natural: yen to dollar", ExpectedOutcome::Ok("currency")),
        case("yuan to dollar", Some("currency"), "natural: yuan to dollar", ExpectedOutcome::Ok("currency")),
        case("rupee to dollar", Some("currency"), "natural: rupee to dollar", ExpectedOutcome::Ok("currency")),
        case("100 rupees to dollars", Some("currency"), "natural: rupees to dollars", ExpectedOutcome::Ok("currency")),
        case("dollar to pound", Some("currency"), "natural: dollar to pound", ExpectedOutcome::Ok("currency")),
        case("dollar to yen", Some("currency"), "natural: dollar to yen", ExpectedOutcome::Ok("currency")),
        case("euro to pound", Some("currency"), "natural: euro to pound", ExpectedOutcome::Ok("currency")),
        case("euro to rupee", Some("currency"), "natural: euro to rupee", ExpectedOutcome::Ok("currency")),
        case("pound to rupee", Some("currency"), "natural: pound to rupee", ExpectedOutcome::Ok("currency")),

        // CURRENCY - Various Amounts
        case("0.5 usd to inr", Some("currency"), "fractional amount", ExpectedOutcome::Ok("currency")),
        case("0.01 usd to inr", Some("currency"), "one cent to INR", ExpectedOutcome::Ok("currency")),
        case("10000 usd to inr", Some("currency"), "large amount", ExpectedOutcome::Ok("currency")),
        case("1000000 usd to inr", Some("currency"), "million USD to INR", ExpectedOutcome::Ok("currency")),
        case("89 usd to inr", Some("currency"), "89 USD to INR", ExpectedOutcome::Ok("currency")),
        case("1.50 usd to eur", Some("currency"), "decimal amount", ExpectedOutcome::Ok("currency")),
        case("99.99 usd to gbp", Some("currency"), "decimal amount GBP", ExpectedOutcome::Ok("currency")),
        case("250 eur to jpy", Some("currency"), "EUR to JPY", ExpectedOutcome::Ok("currency")),
        case("5000 gbp to inr", Some("currency"), "GBP to INR", ExpectedOutcome::Ok("currency")),

        // CURRENCY - Natural Language Variations
        case("convert 100 usd to inr", Some("currency"), "natural: convert X to Y", ExpectedOutcome::Ok("currency")),
        case("how much is 100 usd in inr", Some("currency"), "natural: how much is X in Y", ExpectedOutcome::Ok("currency")),
        case("100 usd in inr", Some("currency"), "natural: X in Y", ExpectedOutcome::Ok("currency")),
        case("100 usd in rupees", Some("currency"), "natural: X in rupees", ExpectedOutcome::Ok("currency")),
        case("whats 50 usd in euros", Some("currency"), "natural: whats X in Y", ExpectedOutcome::Ok("currency")),
        case("$100 to inr", Some("currency"), "natural: $ symbol", ExpectedOutcome::Ok("currency")),
        case("$50 to euros", Some("currency"), "natural: $ to euros", ExpectedOutcome::Ok("currency")),
    ]
}

#[test]
fn imported_super_calculator_currency_cases() {
    block_on(run_cases("currency", cases()));
}
