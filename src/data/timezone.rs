use chrono_tz::Tz;
use papaya::HashMap;
use std::sync::LazyLock;

// --- CITY/REGION TO IANA TIMEZONE ---
pub static TIMEZONE_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let map = HashMap::new();
    let pinned = map.pin();

    // Americas
    pinned.insert("new york", "America/New_York");
    pinned.insert("nyc", "America/New_York");
    pinned.insert("new york city", "America/New_York");
    pinned.insert("manhattan", "America/New_York");
    pinned.insert("boston", "America/New_York");
    pinned.insert("philadelphia", "America/New_York");
    pinned.insert("miami", "America/New_York");
    pinned.insert("atlanta", "America/New_York");
    pinned.insert("washington", "America/New_York");
    pinned.insert("washington dc", "America/New_York");
    pinned.insert("dc", "America/New_York");
    pinned.insert("detroit", "America/New_York");
    pinned.insert("orlando", "America/New_York");
    pinned.insert("charlotte", "America/New_York");
    pinned.insert("pittsburgh", "America/New_York");
    pinned.insert("raleigh", "America/New_York");
    pinned.insert("chicago", "America/Chicago");
    pinned.insert("dallas", "America/Chicago");
    pinned.insert("houston", "America/Chicago");
    pinned.insert("austin", "America/Chicago");
    pinned.insert("san antonio", "America/Chicago");
    pinned.insert("nashville", "America/Chicago");
    pinned.insert("memphis", "America/Chicago");
    pinned.insert("milwaukee", "America/Chicago");
    pinned.insert("minneapolis", "America/Chicago");
    pinned.insert("kansas city", "America/Chicago");
    pinned.insert("new orleans", "America/Chicago");
    pinned.insert("oklahoma city", "America/Chicago");
    pinned.insert("madison", "America/Chicago");
    pinned.insert("indianapolis", "America/Indiana/Indianapolis");
    pinned.insert("denver", "America/Denver");
    pinned.insert("phoenix", "America/Phoenix");
    pinned.insert("salt lake city", "America/Denver");
    pinned.insert("albuquerque", "America/Denver");
    pinned.insert("las vegas", "America/Los_Angeles");
    pinned.insert("los angeles", "America/Los_Angeles");
    pinned.insert("la", "America/Los_Angeles");
    pinned.insert("san francisco", "America/Los_Angeles");
    pinned.insert("sf", "America/Los_Angeles");
    pinned.insert("seattle", "America/Los_Angeles");
    pinned.insert("portland", "America/Los_Angeles");
    pinned.insert("san diego", "America/Los_Angeles");
    pinned.insert("san jose", "America/Los_Angeles");
    pinned.insert("sacramento", "America/Los_Angeles");
    pinned.insert("oakland", "America/Los_Angeles");
    pinned.insert("anchorage", "America/Anchorage");
    pinned.insert("alaska", "America/Anchorage");
    pinned.insert("honolulu", "Pacific/Honolulu");
    pinned.insert("hawaii", "Pacific/Honolulu");
    pinned.insert("toronto", "America/Toronto");
    pinned.insert("montreal", "America/Toronto");
    pinned.insert("ottawa", "America/Toronto");
    pinned.insert("vancouver", "America/Vancouver");
    pinned.insert("calgary", "America/Edmonton");
    pinned.insert("edmonton", "America/Edmonton");
    pinned.insert("winnipeg", "America/Winnipeg");
    pinned.insert("halifax", "America/Halifax");
    pinned.insert("mexico city", "America/Mexico_City");
    pinned.insert("guadalajara", "America/Mexico_City");
    pinned.insert("monterrey", "America/Monterrey");
    pinned.insert("sao paulo", "America/Sao_Paulo");
    pinned.insert("são paulo", "America/Sao_Paulo");
    pinned.insert("rio", "America/Sao_Paulo");
    pinned.insert("rio de janeiro", "America/Sao_Paulo");
    pinned.insert("brasilia", "America/Sao_Paulo");
    pinned.insert("buenos aires", "America/Argentina/Buenos_Aires");
    pinned.insert("argentina", "America/Argentina/Buenos_Aires");
    pinned.insert("lima", "America/Lima");
    pinned.insert("peru", "America/Lima");
    pinned.insert("bogota", "America/Bogota");
    pinned.insert("colombia", "America/Bogota");
    pinned.insert("santiago", "America/Santiago");
    pinned.insert("chile", "America/Santiago");
    pinned.insert("caracas", "America/Caracas");
    pinned.insert("quito", "America/Guayaquil");

    // Europe
    pinned.insert("london", "Europe/London");
    pinned.insert("uk", "Europe/London");
    pinned.insert("united kingdom", "Europe/London");
    pinned.insert("england", "Europe/London");
    pinned.insert("britain", "Europe/London");
    pinned.insert("edinburgh", "Europe/London");
    pinned.insert("manchester", "Europe/London");
    pinned.insert("birmingham", "Europe/London");
    pinned.insert("glasgow", "Europe/London");
    pinned.insert("paris", "Europe/Paris");
    pinned.insert("france", "Europe/Paris");
    pinned.insert("lyon", "Europe/Paris");
    pinned.insert("marseille", "Europe/Paris");
    pinned.insert("berlin", "Europe/Berlin");
    pinned.insert("germany", "Europe/Berlin");
    pinned.insert("munich", "Europe/Berlin");
    pinned.insert("frankfurt", "Europe/Berlin");
    pinned.insert("hamburg", "Europe/Berlin");
    pinned.insert("amsterdam", "Europe/Amsterdam");
    pinned.insert("netherlands", "Europe/Amsterdam");
    pinned.insert("brussels", "Europe/Brussels");
    pinned.insert("belgium", "Europe/Brussels");
    pinned.insert("madrid", "Europe/Madrid");
    pinned.insert("spain", "Europe/Madrid");
    pinned.insert("barcelona", "Europe/Madrid");
    pinned.insert("rome", "Europe/Rome");
    pinned.insert("italy", "Europe/Rome");
    pinned.insert("milan", "Europe/Rome");
    pinned.insert("vienna", "Europe/Vienna");
    pinned.insert("austria", "Europe/Vienna");
    pinned.insert("zurich", "Europe/Zurich");
    pinned.insert("geneva", "Europe/Zurich");
    pinned.insert("switzerland", "Europe/Zurich");
    pinned.insert("lisbon", "Europe/Lisbon");
    pinned.insert("portugal", "Europe/Lisbon");
    pinned.insert("dublin", "Europe/Dublin");
    pinned.insert("ireland", "Europe/Dublin");
    pinned.insert("copenhagen", "Europe/Copenhagen");
    pinned.insert("denmark", "Europe/Copenhagen");
    pinned.insert("stockholm", "Europe/Stockholm");
    pinned.insert("sweden", "Europe/Stockholm");
    pinned.insert("oslo", "Europe/Oslo");
    pinned.insert("norway", "Europe/Oslo");
    pinned.insert("helsinki", "Europe/Helsinki");
    pinned.insert("finland", "Europe/Helsinki");
    pinned.insert("warsaw", "Europe/Warsaw");
    pinned.insert("poland", "Europe/Warsaw");
    pinned.insert("krakow", "Europe/Warsaw");
    pinned.insert("prague", "Europe/Prague");
    pinned.insert("czech republic", "Europe/Prague");
    pinned.insert("czechia", "Europe/Prague");
    pinned.insert("budapest", "Europe/Budapest");
    pinned.insert("hungary", "Europe/Budapest");
    pinned.insert("bucharest", "Europe/Bucharest");
    pinned.insert("romania", "Europe/Bucharest");
    pinned.insert("athens", "Europe/Athens");
    pinned.insert("greece", "Europe/Athens");
    pinned.insert("istanbul", "Europe/Istanbul");
    pinned.insert("turkey", "Europe/Istanbul");
    pinned.insert("ankara", "Europe/Istanbul");
    pinned.insert("moscow", "Europe/Moscow");
    pinned.insert("russia", "Europe/Moscow");
    pinned.insert("st petersburg", "Europe/Moscow");
    pinned.insert("saint petersburg", "Europe/Moscow");
    pinned.insert("kyiv", "Europe/Kyiv");
    pinned.insert("kiev", "Europe/Kyiv");
    pinned.insert("ukraine", "Europe/Kyiv");
    pinned.insert("belgrade", "Europe/Belgrade");
    pinned.insert("serbia", "Europe/Belgrade");
    pinned.insert("sofia", "Europe/Sofia");
    pinned.insert("bulgaria", "Europe/Sofia");
    pinned.insert("zagreb", "Europe/Zagreb");
    pinned.insert("croatia", "Europe/Zagreb");
    pinned.insert("bratislava", "Europe/Bratislava");
    pinned.insert("slovakia", "Europe/Bratislava");
    pinned.insert("tallinn", "Europe/Tallinn");
    pinned.insert("estonia", "Europe/Tallinn");
    pinned.insert("riga", "Europe/Riga");
    pinned.insert("latvia", "Europe/Riga");
    pinned.insert("vilnius", "Europe/Vilnius");
    pinned.insert("lithuania", "Europe/Vilnius");
    pinned.insert("luxembourg", "Europe/Luxembourg");

    // Asia
    pinned.insert("tokyo", "Asia/Tokyo");
    pinned.insert("japan", "Asia/Tokyo");
    pinned.insert("osaka", "Asia/Tokyo");
    pinned.insert("seoul", "Asia/Seoul");
    pinned.insert("south korea", "Asia/Seoul");
    pinned.insert("korea", "Asia/Seoul");
    pinned.insert("beijing", "Asia/Shanghai");
    pinned.insert("shanghai", "Asia/Shanghai");
    pinned.insert("china", "Asia/Shanghai");
    pinned.insert("guangzhou", "Asia/Shanghai");
    pinned.insert("shenzhen", "Asia/Shanghai");
    pinned.insert("hong kong", "Asia/Hong_Kong");
    pinned.insert("hongkong", "Asia/Hong_Kong");
    pinned.insert("taipei", "Asia/Taipei");
    pinned.insert("taiwan", "Asia/Taipei");
    pinned.insert("singapore", "Asia/Singapore");
    pinned.insert("bangkok", "Asia/Bangkok");
    pinned.insert("thailand", "Asia/Bangkok");
    pinned.insert("kuala lumpur", "Asia/Kuala_Lumpur");
    pinned.insert("kl", "Asia/Kuala_Lumpur");
    pinned.insert("malaysia", "Asia/Kuala_Lumpur");
    pinned.insert("jakarta", "Asia/Jakarta");
    pinned.insert("indonesia", "Asia/Jakarta");
    pinned.insert("manila", "Asia/Manila");
    pinned.insert("philippines", "Asia/Manila");
    pinned.insert("hanoi", "Asia/Ho_Chi_Minh");
    pinned.insert("ho chi minh", "Asia/Ho_Chi_Minh");
    pinned.insert("vietnam", "Asia/Ho_Chi_Minh");
    pinned.insert("saigon", "Asia/Ho_Chi_Minh");
    pinned.insert("mumbai", "Asia/Kolkata");
    pinned.insert("delhi", "Asia/Kolkata");
    pinned.insert("new delhi", "Asia/Kolkata");
    pinned.insert("india", "Asia/Kolkata");
    pinned.insert("bangalore", "Asia/Kolkata");
    pinned.insert("bengaluru", "Asia/Kolkata");
    pinned.insert("hyderabad", "Asia/Kolkata");
    pinned.insert("chennai", "Asia/Kolkata");
    pinned.insert("kolkata", "Asia/Kolkata");
    pinned.insert("pune", "Asia/Kolkata");
    pinned.insert("ahmedabad", "Asia/Kolkata");
    pinned.insert("karachi", "Asia/Karachi");
    pinned.insert("lahore", "Asia/Karachi");
    pinned.insert("islamabad", "Asia/Karachi");
    pinned.insert("pakistan", "Asia/Karachi");
    pinned.insert("dhaka", "Asia/Dhaka");
    pinned.insert("bangladesh", "Asia/Dhaka");
    pinned.insert("colombo", "Asia/Colombo");
    pinned.insert("sri lanka", "Asia/Colombo");
    pinned.insert("kathmandu", "Asia/Kathmandu");
    pinned.insert("nepal", "Asia/Kathmandu");
    pinned.insert("dubai", "Asia/Dubai");
    pinned.insert("abu dhabi", "Asia/Dubai");
    pinned.insert("uae", "Asia/Dubai");
    pinned.insert("riyadh", "Asia/Riyadh");
    pinned.insert("saudi arabia", "Asia/Riyadh");
    pinned.insert("jeddah", "Asia/Riyadh");
    pinned.insert("doha", "Asia/Qatar");
    pinned.insert("qatar", "Asia/Qatar");
    pinned.insert("muscat", "Asia/Muscat");
    pinned.insert("oman", "Asia/Muscat");
    pinned.insert("kuwait", "Asia/Kuwait");
    pinned.insert("bahrain", "Asia/Bahrain");
    pinned.insert("tehran", "Asia/Tehran");
    pinned.insert("iran", "Asia/Tehran");
    pinned.insert("baghdad", "Asia/Baghdad");
    pinned.insert("iraq", "Asia/Baghdad");
    pinned.insert("jerusalem", "Asia/Jerusalem");
    pinned.insert("tel aviv", "Asia/Jerusalem");
    pinned.insert("israel", "Asia/Jerusalem");
    pinned.insert("amman", "Asia/Amman");
    pinned.insert("jordan", "Asia/Amman");
    pinned.insert("beirut", "Asia/Beirut");
    pinned.insert("lebanon", "Asia/Beirut");
    pinned.insert("kabul", "Asia/Kabul");
    pinned.insert("afghanistan", "Asia/Kabul");
    pinned.insert("tashkent", "Asia/Tashkent");
    pinned.insert("uzbekistan", "Asia/Tashkent");
    pinned.insert("almaty", "Asia/Almaty");
    pinned.insert("kazakhstan", "Asia/Almaty");
    pinned.insert("yangon", "Asia/Yangon");
    pinned.insert("myanmar", "Asia/Yangon");
    pinned.insert("phnom_penh", "Asia/Phnom_Penh");
    pinned.insert("cambodia", "Asia/Phnom_Penh");

    // Africa
    pinned.insert("cairo", "Africa/Cairo");
    pinned.insert("egypt", "Africa/Cairo");
    pinned.insert("lagos", "Africa/Lagos");
    pinned.insert("nigeria", "Africa/Lagos");
    pinned.insert("nairobi", "Africa/Nairobi");
    pinned.insert("kenya", "Africa/Nairobi");
    pinned.insert("johannesburg", "Africa/Johannesburg");
    pinned.insert("south africa", "Africa/Johannesburg");
    pinned.insert("cape town", "Africa/Johannesburg");
    pinned.insert("accra", "Africa/Accra");
    pinned.insert("ghana", "Africa/Accra");
    pinned.insert("addis_ababa", "Africa/Addis_Ababa");
    pinned.insert("ethiopia", "Africa/Addis_Ababa");
    pinned.insert("addis ababa", "Africa/Addis_Ababa");
    pinned.insert("casablanca", "Africa/Casablanca");
    pinned.insert("morocco", "Africa/Casablanca");
    pinned.insert("tunis", "Africa/Tunis");
    pinned.insert("tunisia", "Africa/Tunis");
    pinned.insert("algiers", "Africa/Algiers");
    pinned.insert("algeria", "Africa/Algiers");
    pinned.insert("dar es salaam", "Africa/Dar_es_Salaam");
    pinned.insert("tanzania", "Africa/Dar_es_Salaam");
    pinned.insert("kampala", "Africa/Kampala");
    pinned.insert("uganda", "Africa/Kampala");
    pinned.insert("kigali", "Africa/Kigali");
    pinned.insert("rwanda", "Africa/Kigali");

    // Oceania
    pinned.insert("sydney", "Australia/Sydney");
    pinned.insert("australia", "Australia/Sydney");
    pinned.insert("melbourne", "Australia/Melbourne");
    pinned.insert("brisbane", "Australia/Brisbane");
    pinned.insert("perth", "Australia/Perth");
    pinned.insert("adelaide", "Australia/Adelaide");
    pinned.insert("auckland", "Pacific/Auckland");
    pinned.insert("new zealand", "Pacific/Auckland");
    pinned.insert("wellington", "Pacific/Auckland");
    pinned.insert("fiji", "Pacific/Fiji");
    pinned.insert("suva", "Pacific/Fiji");

    drop(pinned);
    map
});

// --- TIMEZONE ABBREVIATIONS ---
pub static TZ_ABBREVIATIONS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let map = HashMap::new();
    let pinned = map.pin();

    pinned.insert("utc", "UTC");
    pinned.insert("gmt", "UTC");
    pinned.insert("est", "America/New_York");
    pinned.insert("edt", "America/New_York");
    pinned.insert("et", "America/New_York");
    pinned.insert("cst", "America/Chicago");
    pinned.insert("cdt", "America/Chicago");
    pinned.insert("ct", "America/Chicago");
    pinned.insert("mst", "America/Denver");
    pinned.insert("mdt", "America/Denver");
    pinned.insert("mt", "America/Denver");
    pinned.insert("pst", "America/Los_Angeles");
    pinned.insert("pdt", "America/Los_Angeles");
    pinned.insert("pt", "America/Los_Angeles");
    pinned.insert("akst", "America/Anchorage");
    pinned.insert("akdt", "America/Anchorage");
    pinned.insert("hst", "Pacific/Honolulu");
    pinned.insert("ist", "Asia/Kolkata");
    pinned.insert("cet", "Europe/Paris");
    pinned.insert("cest", "Europe/Paris");
    pinned.insert("eet", "Europe/Athens");
    pinned.insert("eest", "Europe/Athens");
    pinned.insert("wet", "Europe/Lisbon");
    pinned.insert("west", "Europe/Lisbon");
    pinned.insert("bst", "Europe/London");
    pinned.insert("gst", "Asia/Dubai");
    pinned.insert("jst", "Asia/Tokyo");
    pinned.insert("kst", "Asia/Seoul");
    pinned.insert("cst_china", "Asia/Shanghai");
    pinned.insert("hkt", "Asia/Hong_Kong");
    pinned.insert("sgt", "Asia/Singapore");
    pinned.insert("ict", "Asia/Bangkok");
    pinned.insert("wib", "Asia/Jakarta");
    pinned.insert("pht", "Asia/Manila");
    pinned.insert("nzst", "Pacific/Auckland");
    pinned.insert("nzdt", "Pacific/Auckland");
    pinned.insert("aest", "Australia/Sydney");
    pinned.insert("aedt", "Australia/Sydney");
    pinned.insert("acst", "Australia/Adelaide");
    pinned.insert("acdt", "Australia/Adelaide");
    pinned.insert("awst", "Australia/Perth");
    pinned.insert("ast", "America/Halifax");
    pinned.insert("nst", "America/St_Johns");
    pinned.insert("brt", "America/Sao_Paulo");
    pinned.insert("art", "America/Argentina/Buenos_Aires");
    pinned.insert("msk", "Europe/Moscow");
    pinned.insert("irst", "Asia/Tehran");
    pinned.insert("pkt", "Asia/Karachi");
    pinned.insert("npt", "Asia/Kathmandu");
    pinned.insert("bdt", "Asia/Dhaka");
    pinned.insert("mmt", "Asia/Yangon");
    pinned.insert("cat", "Africa/Nairobi");
    pinned.insert("eat", "Africa/Nairobi");
    pinned.insert("wat", "Africa/Lagos");
    pinned.insert("sast", "Africa/Johannesburg");

    drop(pinned);
    map
});

// --- COUNTRY TO COMMON TIMEZONE ---
pub static COUNTRY_TZ: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let map = HashMap::new();
    let pinned = map.pin();

    pinned.insert("us", "America/New_York");
    pinned.insert("usa", "America/New_York");
    pinned.insert("united states", "America/New_York");
    pinned.insert("america", "America/New_York");
    pinned.insert("canada", "America/Toronto");
    pinned.insert("mexico", "America/Mexico_City");
    pinned.insert("brazil", "America/Sao_Paulo");
    pinned.insert("colombia", "America/Bogota");
    pinned.insert("peru", "America/Lima");
    pinned.insert("chile", "America/Santiago");
    pinned.insert("uk", "Europe/London");
    pinned.insert("united kingdom", "Europe/London");
    pinned.insert("britain", "Europe/London");
    pinned.insert("england", "Europe/London");
    pinned.insert("france", "Europe/Paris");
    pinned.insert("germany", "Europe/Berlin");
    pinned.insert("italy", "Europe/Rome");
    pinned.insert("spain", "Europe/Madrid");
    pinned.insert("netherlands", "Europe/Amsterdam");
    pinned.insert("belgium", "Europe/Brussels");
    pinned.insert("switzerland", "Europe/Zurich");
    pinned.insert("portugal", "Europe/Lisbon");
    pinned.insert("ireland", "Europe/Dublin");
    pinned.insert("austria", "Europe/Vienna");
    pinned.insert("sweden", "Europe/Stockholm");
    pinned.insert("norway", "Europe/Oslo");
    pinned.insert("denmark", "Europe/Copenhagen");
    pinned.insert("finland", "Europe/Helsinki");
    pinned.insert("poland", "Europe/Warsaw");
    pinned.insert("czechia", "Europe/Prague");
    pinned.insert("hungary", "Europe/Budapest");
    pinned.insert("romania", "Europe/Bucharest");
    pinned.insert("greece", "Europe/Athens");
    pinned.insert("turkey", "Europe/Istanbul");
    pinned.insert("russia", "Europe/Moscow");
    pinned.insert("ukraine", "Europe/Kyiv");
    pinned.insert("india", "Asia/Kolkata");
    pinned.insert("pakistan", "Asia/Karachi");
    pinned.insert("bangladesh", "Asia/Dhaka");
    pinned.insert("china", "Asia/Shanghai");
    pinned.insert("japan", "Asia/Tokyo");
    pinned.insert("south korea", "Asia/Seoul");
    pinned.insert("korea", "Asia/Seoul");
    pinned.insert("indonesia", "Asia/Jakarta");
    pinned.insert("malaysia", "Asia/Kuala_Lumpur");
    pinned.insert("singapore", "Asia/Singapore");
    pinned.insert("thailand", "Asia/Bangkok");
    pinned.insert("vietnam", "Asia/Ho_Chi_Minh");
    pinned.insert("philippines", "Asia/Manila");
    pinned.insert("uae", "Asia/Dubai");
    pinned.insert("saudi arabia", "Asia/Riyadh");
    pinned.insert("qatar", "Asia/Qatar");
    pinned.insert("israel", "Asia/Jerusalem");
    pinned.insert("iran", "Asia/Tehran");
    pinned.insert("iraq", "Asia/Baghdad");
    pinned.insert("australia", "Australia/Sydney");
    pinned.insert("new zealand", "Pacific/Auckland");
    pinned.insert("egypt", "Africa/Cairo");
    pinned.insert("nigeria", "Africa/Lagos");
    pinned.insert("kenya", "Africa/Nairobi");
    pinned.insert("south africa", "Africa/Johannesburg");
    pinned.insert("ghana", "Africa/Accra");
    pinned.insert("ethiopia", "Africa/Addis_Ababa");

    drop(pinned);
    map
});

pub fn resolve_timezone(name: &str) -> Option<String> {
    let cleaned = name.trim();
    let lower = cleaned.to_lowercase();

    if lower.contains('/') {
        if cleaned.parse::<Tz>().is_ok() {
            return Some(cleaned.to_string());
        }
    }

    if let Some(&val) = TZ_ABBREVIATIONS.pin().get(lower.as_str()) {
        return Some(val.to_string());
    }

    if let Some(&val) = TIMEZONE_MAP.pin().get(lower.as_str()) {
        return Some(val.to_string());
    }

    if let Some(&val) = COUNTRY_TZ.pin().get(lower.as_str()) {
        return Some(val.to_string());
    }

    None
}
