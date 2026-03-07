#[path = "support/mod.rs"]
mod support;

use support::{Case, ExpectedOutcome, block_on, case, run_cases};

pub fn cases() -> Vec<Case> {
    vec![

        // CRYPTO - Major Coins
        case("btc to usd", Some("crypto"), "BTC to USD rate", ExpectedOutcome::Ok("crypto")),
        case("1 btc to usd", Some("crypto"), "1 BTC to USD", ExpectedOutcome::Ok("crypto")),
        case("0.5 btc to usd", Some("crypto"), "0.5 BTC to USD", ExpectedOutcome::Ok("crypto")),
        case("0.01 btc to usd", Some("crypto"), "0.01 BTC to USD", ExpectedOutcome::Ok("crypto")),
        case("eth to usd", Some("crypto"), "ETH to USD rate", ExpectedOutcome::Ok("crypto")),
        case("1 eth to usd", Some("crypto"), "1 ETH to USD", ExpectedOutcome::Ok("crypto")),
        case("1.5 eth to usd", Some("crypto"), "1.5 ETH to USD", ExpectedOutcome::Ok("crypto")),
        case("sol to usd", Some("crypto"), "SOL to USD", ExpectedOutcome::Ok("crypto")),
        case("1 sol to usd", Some("crypto"), "1 SOL to USD", ExpectedOutcome::Ok("crypto")),
        case("xrp to usd", Some("crypto"), "XRP to USD", ExpectedOutcome::Ok("crypto")),
        case("100 xrp to usd", Some("crypto"), "100 XRP to USD", ExpectedOutcome::Ok("crypto")),
        case("doge to usd", Some("crypto"), "DOGE to USD", ExpectedOutcome::Ok("crypto")),
        case("1000 doge to usd", Some("crypto"), "1000 DOGE to USD", ExpectedOutcome::Ok("crypto")),
        case("ada to usd", Some("crypto"), "ADA to USD", ExpectedOutcome::Ok("crypto")),
        case("matic to usd", Some("crypto"), "MATIC to USD", ExpectedOutcome::Ok("crypto")),
        case("dot to usd", Some("crypto"), "DOT to USD", ExpectedOutcome::Ok("crypto")),
        case("ltc to usd", Some("crypto"), "LTC to USD", ExpectedOutcome::Ok("crypto")),
        case("avax to usd", Some("crypto"), "AVAX to USD", ExpectedOutcome::Ok("crypto")),
        case("link to usd", Some("crypto"), "LINK to USD", ExpectedOutcome::Ok("crypto")),
        case("uni to usd", Some("crypto"), "UNI to USD", ExpectedOutcome::Ok("crypto")),
        case("atom to usd", Some("crypto"), "ATOM to USD", ExpectedOutcome::Ok("crypto")),
        case("xlm to usd", Some("crypto"), "XLM to USD", ExpectedOutcome::Ok("crypto")),
        case("ton to usd", Some("crypto"), "TON to USD", ExpectedOutcome::Ok("crypto")),
        case("shib to usd", Some("crypto"), "SHIB to USD", ExpectedOutcome::Ok("crypto")),
        case("bnb to usd", Some("crypto"), "BNB to USD", ExpectedOutcome::Ok("crypto")),
        case("usdt to usd", Some("crypto"), "USDT to USD", ExpectedOutcome::Ok("crypto")),
        case("usdc to usd", Some("crypto"), "USDC to USD", ExpectedOutcome::Ok("crypto")),

        // CRYPTO - USD to Crypto (reverse)
        case("100 usd to btc", Some("crypto"), "100 USD to BTC", ExpectedOutcome::Ok("crypto")),
        case("1000 usd to btc", Some("crypto"), "1000 USD to BTC", ExpectedOutcome::Ok("crypto")),
        case("100 usd to eth", Some("crypto"), "100 USD to ETH", ExpectedOutcome::Ok("crypto")),
        case("50 usd to sol", Some("crypto"), "50 USD to SOL", ExpectedOutcome::Ok("crypto")),
        case("100 usd to xrp", Some("crypto"), "100 USD to XRP", ExpectedOutcome::Ok("crypto")),
        case("10 usd to doge", Some("crypto"), "10 USD to DOGE", ExpectedOutcome::Ok("crypto")),

        // CRYPTO - To Non-USD Fiat
        case("1 btc to inr", Some("crypto"), "BTC to INR", ExpectedOutcome::Ok("crypto")),
        case("1 eth to inr", Some("crypto"), "ETH to INR", ExpectedOutcome::Ok("crypto")),
        case("1.5 eth to inr", Some("crypto"), "1.5 ETH to INR", ExpectedOutcome::Ok("crypto")),
        case("doge to inr", Some("crypto"), "DOGE to INR", ExpectedOutcome::Ok("crypto")),
        case("1 btc to eur", Some("crypto"), "BTC to EUR", ExpectedOutcome::Ok("crypto")),
        case("1 btc to gbp", Some("crypto"), "BTC to GBP", ExpectedOutcome::Ok("crypto")),
        case("1 eth to jpy", Some("crypto"), "ETH to JPY", ExpectedOutcome::Ok("crypto")),
        case("sol to inr", Some("crypto"), "SOL to INR", ExpectedOutcome::Ok("crypto")),
        case("xrp to inr", Some("crypto"), "XRP to INR", ExpectedOutcome::Ok("crypto")),

        // CRYPTO - Full Names
        case("bitcoin to usd", Some("crypto"), "natural: bitcoin to usd", ExpectedOutcome::Ok("crypto")),
        case("1 bitcoin to usd", Some("crypto"), "natural: 1 bitcoin to usd", ExpectedOutcome::Ok("crypto")),
        case("ethereum to usd", Some("crypto"), "natural: ethereum to usd", ExpectedOutcome::Ok("crypto")),
        case("1 ethereum to usd", Some("crypto"), "natural: 1 ethereum to usd", ExpectedOutcome::Ok("crypto")),
        case("solana to usd", Some("crypto"), "natural: solana to usd", ExpectedOutcome::Ok("crypto")),
        case("dogecoin to usd", Some("crypto"), "natural: dogecoin to usd", ExpectedOutcome::Ok("crypto")),
        case("litecoin to usd", Some("crypto"), "natural: litecoin to usd", ExpectedOutcome::Ok("crypto")),
        case("bitcoin to inr", Some("crypto"), "natural: bitcoin to inr", ExpectedOutcome::Ok("crypto")),
        case("ethereum to inr", Some("crypto"), "natural: ethereum to inr", ExpectedOutcome::Ok("crypto")),
        case("bitcoin to rupee", Some("crypto"), "natural: bitcoin to rupee", ExpectedOutcome::Ok("crypto")),
        case("100 usd to bitcoin", Some("crypto"), "natural: usd to bitcoin", ExpectedOutcome::Ok("crypto")),
        case("ethereum to rupees", Some("crypto"), "natural: ethereum to rupees", ExpectedOutcome::Ok("crypto")),

        // CRYPTO - Crypto to Crypto
        case("1 btc to eth", Some("crypto"), "BTC to ETH", ExpectedOutcome::Ok("crypto")),
        case("10 eth to btc", Some("crypto"), "ETH to BTC", ExpectedOutcome::Ok("crypto")),
        case("1 btc to sol", Some("crypto"), "BTC to SOL", ExpectedOutcome::Ok("crypto")),
        case("100 sol to eth", Some("crypto"), "SOL to ETH", ExpectedOutcome::Ok("crypto")),
        case("1000 doge to btc", Some("crypto"), "DOGE to BTC", ExpectedOutcome::Ok("crypto")),
    ]
}

#[test]
fn imported_super_calculator_crypto_cases() {
    block_on(run_cases("crypto", cases()));
}
