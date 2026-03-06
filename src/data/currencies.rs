use papaya::HashMap;
use std::sync::LazyLock;

// --- FIAT CURRENCIES ---
pub static FIAT_CURRENCIES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let map = HashMap::new();
    let pinned = map.pin();

    // Major
    pinned.insert("USD", "US Dollar");
    pinned.insert("EUR", "Euro");
    pinned.insert("GBP", "British Pound");
    pinned.insert("JPY", "Japanese Yen");
    pinned.insert("CHF", "Swiss Franc");
    pinned.insert("CAD", "Canadian Dollar");
    pinned.insert("AUD", "Australian Dollar");
    pinned.insert("NZD", "New Zealand Dollar");
    pinned.insert("CNY", "Chinese Yuan");
    pinned.insert("HKD", "Hong Kong Dollar");
    pinned.insert("SGD", "Singapore Dollar");
    pinned.insert("SEK", "Swedish Krona");
    pinned.insert("NOK", "Norwegian Krone");
    pinned.insert("DKK", "Danish Krone");
    pinned.insert("KRW", "South Korean Won");
    pinned.insert("TWD", "Taiwan Dollar");
    // Asia
    pinned.insert("INR", "Indian Rupee");
    pinned.insert("PKR", "Pakistani Rupee");
    pinned.insert("BDT", "Bangladeshi Taka");
    pinned.insert("LKR", "Sri Lankan Rupee");
    pinned.insert("NPR", "Nepalese Rupee");
    pinned.insert("IDR", "Indonesian Rupiah");
    pinned.insert("MYR", "Malaysian Ringgit");
    pinned.insert("THB", "Thai Baht");
    pinned.insert("VND", "Vietnamese Dong");
    pinned.insert("PHP", "Philippine Peso");
    pinned.insert("MMK", "Myanmar Kyat");
    pinned.insert("KHR", "Cambodian Riel");
    pinned.insert("LAK", "Lao Kip");
    pinned.insert("MNT", "Mongolian Tugrik");
    pinned.insert("KZT", "Kazakh Tenge");
    pinned.insert("UZS", "Uzbekistani Som");
    pinned.insert("GEL", "Georgian Lari");
    pinned.insert("AZN", "Azerbaijani Manat");
    pinned.insert("AMD", "Armenian Dram");
    // Middle East
    pinned.insert("AED", "UAE Dirham");
    pinned.insert("SAR", "Saudi Riyal");
    pinned.insert("QAR", "Qatari Riyal");
    pinned.insert("OMR", "Omani Rial");
    pinned.insert("KWD", "Kuwaiti Dinar");
    pinned.insert("BHD", "Bahraini Dinar");
    pinned.insert("ILS", "Israeli Shekel");
    pinned.insert("JOD", "Jordanian Dinar");
    pinned.insert("LBP", "Lebanese Pound");
    pinned.insert("IQD", "Iraqi Dinar");
    pinned.insert("IRR", "Iranian Rial");
    pinned.insert("TRY", "Turkish Lira");
    // Africa
    pinned.insert("ZAR", "South African Rand");
    pinned.insert("NGN", "Nigerian Naira");
    pinned.insert("EGP", "Egyptian Pound");
    pinned.insert("KES", "Kenyan Shilling");
    pinned.insert("GHS", "Ghanaian Cedi");
    pinned.insert("TZS", "Tanzanian Shilling");
    pinned.insert("UGX", "Ugandan Shilling");
    pinned.insert("MAD", "Moroccan Dirham");
    pinned.insert("TND", "Tunisian Dinar");
    pinned.insert("DZD", "Algerian Dinar");
    pinned.insert("XOF", "West African CFA");
    pinned.insert("XAF", "Central African CFA");
    pinned.insert("ETB", "Ethiopian Birr");
    pinned.insert("RWF", "Rwandan Franc");
    pinned.insert("MUR", "Mauritian Rupee");
    // Americas
    pinned.insert("MXN", "Mexican Peso");
    pinned.insert("BRL", "Brazilian Real");
    pinned.insert("ARS", "Argentine Peso");
    pinned.insert("CLP", "Chilean Peso");
    pinned.insert("COP", "Colombian Peso");
    pinned.insert("PEN", "Peruvian Sol");
    pinned.insert("UYU", "Uruguayan Peso");
    pinned.insert("BOB", "Bolivian Boliviano");
    pinned.insert("PYG", "Paraguayan Guarani");
    pinned.insert("VES", "Venezuelan Bolivar");
    pinned.insert("CRC", "Costa Rican Colon");
    pinned.insert("GTQ", "Guatemalan Quetzal");
    pinned.insert("HNL", "Honduran Lempira");
    pinned.insert("NIO", "Nicaraguan Cordoba");
    pinned.insert("PAB", "Panamanian Balboa");
    pinned.insert("DOP", "Dominican Peso");
    pinned.insert("TTD", "Trinidad Dollar");
    pinned.insert("JMD", "Jamaican Dollar");
    pinned.insert("HTG", "Haitian Gourde");
    pinned.insert("BSD", "Bahamian Dollar");
    pinned.insert("BBD", "Barbadian Dollar");
    pinned.insert("BZD", "Belizean Dollar");
    // Europe
    pinned.insert("PLN", "Polish Zloty");
    pinned.insert("CZK", "Czech Koruna");
    pinned.insert("HUF", "Hungarian Forint");
    pinned.insert("RON", "Romanian Leu");
    pinned.insert("BGN", "Bulgarian Lev");
    pinned.insert("HRK", "Croatian Kuna");
    pinned.insert("RSD", "Serbian Dinar");
    pinned.insert("UAH", "Ukrainian Hryvnia");
    pinned.insert("RUB", "Russian Ruble");
    pinned.insert("BYN", "Belarusian Ruble");
    pinned.insert("MDL", "Moldovan Leu");
    pinned.insert("MKD", "Macedonian Denar");
    pinned.insert("ALL", "Albanian Lek");
    pinned.insert("BAM", "Bosnian Mark");
    pinned.insert("ISK", "Icelandic Krona");
    // Oceania
    pinned.insert("FJD", "Fijian Dollar");
    pinned.insert("PGK", "Papua New Guinean Kina");
    pinned.insert("WST", "Samoan Tala");
    pinned.insert("TOP", "Tongan Paanga");
    pinned.insert("VUV", "Vanuatu Vatu");
    pinned.insert("SBD", "Solomon Islands Dollar");
    // Caribbean
    pinned.insert("XCD", "East Caribbean Dollar");
    pinned.insert("AWG", "Aruban Florin");
    pinned.insert("ANG", "Netherlands Antillean Guilder");
    pinned.insert("CUP", "Cuban Peso");
    pinned.insert("KYD", "Cayman Islands Dollar");
    pinned.insert("BMD", "Bermudian Dollar");
    // Metals
    pinned.insert("XAU", "Gold (troy oz)");
    pinned.insert("XAG", "Silver (troy oz)");
    pinned.insert("XPT", "Platinum (troy oz)");

    drop(pinned);

    map
});

// --- FIAT ALIASES ---
pub static FIAT_ALIASES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let map = HashMap::new();
    let pinned = map.pin();

    pinned.insert("dollar", "USD");
    pinned.insert("dollars", "USD");
    pinned.insert("usd", "USD");
    pinned.insert("$", "USD");
    pinned.insert("us dollar", "USD");
    pinned.insert("us dollars", "USD");
    pinned.insert("euro", "EUR");
    pinned.insert("euros", "EUR");
    pinned.insert("eur", "EUR");
    pinned.insert("€", "EUR");
    pinned.insert("pound", "GBP");
    pinned.insert("pounds", "GBP");
    pinned.insert("gbp", "GBP");
    pinned.insert("sterling", "GBP");
    pinned.insert("£", "GBP");
    pinned.insert("yen", "JPY");
    pinned.insert("jpy", "JPY");
    pinned.insert("¥", "JPY");
    pinned.insert("rupee", "INR");
    pinned.insert("rupees", "INR");
    pinned.insert("inr", "INR");
    pinned.insert("₹", "INR");
    pinned.insert("yuan", "CNY");
    pinned.insert("renminbi", "CNY");
    pinned.insert("rmb", "CNY");
    pinned.insert("cny", "CNY");
    pinned.insert("won", "KRW");
    pinned.insert("krw", "KRW");
    pinned.insert("₩", "KRW");
    pinned.insert("franc", "CHF");
    pinned.insert("chf", "CHF");
    pinned.insert("real", "BRL");
    pinned.insert("reais", "BRL");
    pinned.insert("brl", "BRL");
    pinned.insert("peso", "MXN");
    pinned.insert("mxn", "MXN");
    pinned.insert("lira", "TRY");
    pinned.insert("try", "TRY");
    pinned.insert("₺", "TRY");
    pinned.insert("baht", "THB");
    pinned.insert("thb", "THB");
    pinned.insert("฿", "THB");
    pinned.insert("ringgit", "MYR");
    pinned.insert("myr", "MYR");
    pinned.insert("rand", "ZAR");
    pinned.insert("zar", "ZAR");
    pinned.insert("dirham", "AED");
    pinned.insert("aed", "AED");
    pinned.insert("riyal", "SAR");
    pinned.insert("sar", "SAR");
    pinned.insert("shekel", "ILS");
    pinned.insert("shekels", "ILS");
    pinned.insert("ils", "ILS");
    pinned.insert("₪", "ILS");
    pinned.insert("ruble", "RUB");
    pinned.insert("rubles", "RUB");
    pinned.insert("rub", "RUB");
    pinned.insert("₽", "RUB");
    pinned.insert("zloty", "PLN");
    pinned.insert("pln", "PLN");
    pinned.insert("krona", "SEK");
    pinned.insert("sek", "SEK");
    pinned.insert("krone", "NOK");
    pinned.insert("nok", "NOK");
    pinned.insert("dinar", "KWD");
    pinned.insert("kwd", "KWD");
    pinned.insert("naira", "NGN");
    pinned.insert("ngn", "NGN");
    pinned.insert("₦", "NGN");
    pinned.insert("cedi", "GHS");
    pinned.insert("ghs", "GHS");
    pinned.insert("₵", "GHS");
    pinned.insert("birr", "ETB");
    pinned.insert("etb", "ETB");

    drop(pinned);

    map
});

// --- CRYPTO CURRENCIES ---
pub static CRYPTO_CURRENCIES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let map = HashMap::new();
    let pinned = map.pin();

    pinned.insert("BTC", "Bitcoin");
    pinned.insert("ETH", "Ethereum");
    pinned.insert("SOL", "Solana");
    pinned.insert("XRP", "Ripple");
    pinned.insert("USDT", "Tether");
    pinned.insert("USDC", "USD Coin");
    pinned.insert("BNB", "BNB");
    pinned.insert("DOGE", "Dogecoin");
    pinned.insert("ADA", "Cardano");
    pinned.insert("MATIC", "Polygon");
    pinned.insert("DOT", "Polkadot");
    pinned.insert("LTC", "Litecoin");
    pinned.insert("AVAX", "Avalanche");
    pinned.insert("LINK", "Chainlink");
    pinned.insert("UNI", "Uniswap");
    pinned.insert("ATOM", "Cosmos");
    pinned.insert("XLM", "Stellar");
    pinned.insert("ALGO", "Algorand");
    pinned.insert("FIL", "Filecoin");
    pinned.insert("NEAR", "NEAR Protocol");
    pinned.insert("APT", "Aptos");
    pinned.insert("ARB", "Arbitrum");
    pinned.insert("OP", "Optimism");
    pinned.insert("SUI", "Sui");
    pinned.insert("SHIB", "Shiba Inu");
    pinned.insert("PEPE", "Pepe");
    pinned.insert("TRX", "TRON");
    pinned.insert("TON", "Toncoin");
    pinned.insert("HBAR", "Hedera");
    pinned.insert("ICP", "Internet Computer");
    pinned.insert("VET", "VeChain");
    pinned.insert("AAVE", "Aave");
    pinned.insert("MKR", "Maker");
    pinned.insert("CRV", "Curve");
    pinned.insert("SNX", "Synthetix");
    pinned.insert("COMP", "Compound");
    pinned.insert("SAND", "The Sandbox");
    pinned.insert("MANA", "Decentraland");
    pinned.insert("AXS", "Axie Infinity");
    pinned.insert("GMT", "STEPN");
    pinned.insert("APE", "ApeCoin");
    pinned.insert("FTM", "Fantom");
    pinned.insert("ONE", "Harmony");
    pinned.insert("KAVA", "Kava");
    pinned.insert("ROSE", "Oasis");
    pinned.insert("ZEC", "Zcash");
    pinned.insert("XMR", "Monero");
    pinned.insert("DASH", "Dash");
    pinned.insert("ETC", "Ethereum Classic");
    pinned.insert("BCH", "Bitcoin Cash");
    pinned.insert("BSV", "Bitcoin SV");

    drop(pinned);

    map
});

// --- CRYPTO ALIASES ---
pub static CRYPTO_ALIASES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let map = HashMap::new();
    let pinned = map.pin();

    pinned.insert("bitcoin", "BTC");
    pinned.insert("btc", "BTC");
    pinned.insert("satoshi", "BTC");
    pinned.insert("sats", "BTC");
    pinned.insert("ethereum", "ETH");
    pinned.insert("eth", "ETH");
    pinned.insert("ether", "ETH");
    pinned.insert("solana", "SOL");
    pinned.insert("sol", "SOL");
    pinned.insert("ripple", "XRP");
    pinned.insert("xrp", "XRP");
    pinned.insert("tether", "USDT");
    pinned.insert("usdt", "USDT");
    pinned.insert("usd coin", "USDC");
    pinned.insert("usdc", "USDC");
    pinned.insert("bnb", "BNB");
    pinned.insert("binance", "BNB");
    pinned.insert("dogecoin", "DOGE");
    pinned.insert("doge", "DOGE");
    pinned.insert("cardano", "ADA");
    pinned.insert("ada", "ADA");
    pinned.insert("polygon", "MATIC");
    pinned.insert("matic", "MATIC");
    pinned.insert("polkadot", "DOT");
    pinned.insert("dot", "DOT");
    pinned.insert("litecoin", "LTC");
    pinned.insert("ltc", "LTC");
    pinned.insert("avalanche", "AVAX");
    pinned.insert("avax", "AVAX");
    pinned.insert("chainlink", "LINK");
    pinned.insert("link", "LINK");
    pinned.insert("uniswap", "UNI");
    pinned.insert("uni", "UNI");
    pinned.insert("cosmos", "ATOM");
    pinned.insert("atom", "ATOM");
    pinned.insert("stellar", "XLM");
    pinned.insert("xlm", "XLM");
    pinned.insert("monero", "XMR");
    pinned.insert("xmr", "XMR");
    pinned.insert("zcash", "ZEC");
    pinned.insert("zec", "ZEC");
    pinned.insert("tron", "TRX");
    pinned.insert("trx", "TRX");
    pinned.insert("toncoin", "TON");
    pinned.insert("ton", "TON");
    pinned.insert("bitcoin cash", "BCH");
    pinned.insert("bch", "BCH");
    pinned.insert("ethereum classic", "ETC");
    pinned.insert("etc", "ETC");

    drop(pinned);

    map
});

pub fn resolve_fiat(token: &str) -> Option<String> {
    let upper = token.to_uppercase();
    if FIAT_CURRENCIES.pin().contains_key(upper.as_str()) {
        return Some(upper);
    }

    let lower = token.to_lowercase();
    if let Some(&code) = FIAT_ALIASES.pin().get(lower.as_str()) {
        return Some(code.to_string());
    }

    None
}

pub fn resolve_crypto(token: &str) -> Option<String> {
    let upper = token.to_uppercase();
    if CRYPTO_CURRENCIES.pin().contains_key(upper.as_str()) {
        return Some(upper);
    }

    let lower = token.to_lowercase();
    if let Some(&code) = CRYPTO_ALIASES.pin().get(lower.as_str()) {
        return Some(code.to_string());
    }

    None
}

pub fn is_currency(token: &str) -> bool {
    resolve_fiat(token).is_some()
}

pub fn is_crypto(token: &str) -> bool {
    resolve_crypto(token).is_some()
}
