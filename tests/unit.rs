#[path = "support/mod.rs"]
mod support;

use support::{Case, ExpectedOutcome, block_on, case, run_cases};

pub fn cases() -> Vec<Case> {
    vec![

        // UNIT CONVERSIONS - Length
        case("3 km to m", Some("unit"), "km to meters", ExpectedOutcome::Ok("unit")),
        case("1 m to cm", Some("unit"), "meters to cm", ExpectedOutcome::Ok("unit")),
        case("1 mi to km", Some("unit"), "miles to km", ExpectedOutcome::Ok("unit")),
        case("12 in to cm", Some("unit"), "inches to cm", ExpectedOutcome::Ok("unit")),
        case("6 ft to m", Some("unit"), "feet to meters", ExpectedOutcome::Ok("unit")),
        case("100 yd to m", Some("unit"), "yards to meters", ExpectedOutcome::Ok("unit")),
        case("1 nmi to km", Some("unit"), "nautical miles to km", ExpectedOutcome::Ok("unit")),
        case("1 light year to km", Some("unit"), "light year to km", ExpectedOutcome::Ok("unit")),
        case("1 au to km", Some("unit"), "astronomical unit to km", ExpectedOutcome::Ok("unit")),
        case("1000 mm to m", Some("unit"), "mm to meters", ExpectedOutcome::Ok("unit")),
        case("500 nm to mm", Some("unit"), "nanometers to mm", ExpectedOutcome::Ok("unit")),
        case("1 parsec to light year", Some("unit"), "parsec to light year", ExpectedOutcome::Ok("unit")),
        case("5280 ft to mi", Some("unit"), "5280 feet to miles", ExpectedOutcome::Ok("unit")),
        case("1 km to ft", Some("unit"), "km to feet", ExpectedOutcome::Ok("unit")),
        case("1 cm to in", Some("unit"), "cm to inches", ExpectedOutcome::Ok("unit")),
        case("100 cm to m", Some("unit"), "100 cm to meters", ExpectedOutcome::Ok("unit")),
        case("0.5 km to m", Some("unit"), "half km to meters", ExpectedOutcome::Ok("unit")),
        case("1000000 mm to km", Some("unit"), "million mm to km", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Mass
        case("5 kg to lb", Some("unit"), "kg to pounds", ExpectedOutcome::Ok("unit")),
        case("1 lb to kg", Some("unit"), "pounds to kg", ExpectedOutcome::Ok("unit")),
        case("1000 g to kg", Some("unit"), "grams to kg", ExpectedOutcome::Ok("unit")),
        case("1 tonne to kg", Some("unit"), "tonne to kg", ExpectedOutcome::Ok("unit")),
        case("16 oz to lb", Some("unit"), "ounces to pounds", ExpectedOutcome::Ok("unit")),
        case("1 stone to kg", Some("unit"), "stone to kg", ExpectedOutcome::Ok("unit")),
        case("1 carat to g", Some("unit"), "carat to grams", ExpectedOutcome::Ok("unit")),
        case("1000 mg to g", Some("unit"), "mg to grams", ExpectedOutcome::Ok("unit")),
        case("1 grain to mg", Some("unit"), "grain to mg", ExpectedOutcome::Ok("unit")),
        case("100 lb to kg", Some("unit"), "100 pounds to kg", ExpectedOutcome::Ok("unit")),
        case("2.5 kg to g", Some("unit"), "2.5 kg to grams", ExpectedOutcome::Ok("unit")),
        case("1 kg to oz", Some("unit"), "kg to ounces", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Volume
        case("1 l to ml", Some("unit"), "liters to ml", ExpectedOutcome::Ok("unit")),
        case("1 gallon to l", Some("unit"), "gallon to liters", ExpectedOutcome::Ok("unit")),
        case("1 cup to ml", Some("unit"), "cup to ml", ExpectedOutcome::Ok("unit")),
        case("1 pint to ml", Some("unit"), "pint to ml", ExpectedOutcome::Ok("unit")),
        case("1 quart to l", Some("unit"), "quart to liters", ExpectedOutcome::Ok("unit")),
        case("3 tsp to tbsp", Some("unit"), "teaspoons to tablespoons", ExpectedOutcome::Ok("unit")),
        case("1 fl oz to ml", Some("unit"), "fluid ounce to ml", ExpectedOutcome::Ok("unit")),
        case("1 barrel to gallon", Some("unit"), "barrel to gallons", ExpectedOutcome::Ok("unit")),
        case("500 ml to cup", Some("unit"), "ml to cups", ExpectedOutcome::Ok("unit")),
        case("2 l to pint", Some("unit"), "liters to pints", ExpectedOutcome::Ok("unit")),
        case("1 tbsp to tsp", Some("unit"), "tablespoon to teaspoons", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Area
        case("1 km² to m²", Some("unit"), "sq km to sq meters", ExpectedOutcome::Err),
        case("1 acre to m²", Some("unit"), "acre to sq meters", ExpectedOutcome::Err),
        case("1 hectare to acre", Some("unit"), "hectare to acres", ExpectedOutcome::Ok("unit")),
        case("1 mi² to km²", Some("unit"), "sq miles to sq km", ExpectedOutcome::Err),
        case("1 ft² to m²", Some("unit"), "sq feet to sq meters", ExpectedOutcome::Err),
        case("144 in² to ft²", Some("unit"), "sq inches to sq feet", ExpectedOutcome::Err),
        case("1 yd² to ft²", Some("unit"), "sq yards to sq feet", ExpectedOutcome::Err),
        case("10000 m² to hectare", Some("unit"), "sq meters to hectare", ExpectedOutcome::Err),

        // UNIT CONVERSIONS - Temperature
        case("100 fahrenheit to celsius", Some("unit"), "100F to C", ExpectedOutcome::Ok("unit")),
        case("0 celsius to fahrenheit", Some("unit"), "0C to F", ExpectedOutcome::Ok("unit")),
        case("32 fahrenheit to celsius", Some("unit"), "freezing point F to C", ExpectedOutcome::Ok("unit")),
        case("212 fahrenheit to celsius", Some("unit"), "boiling point F to C", ExpectedOutcome::Ok("unit")),
        case("100 celsius to fahrenheit", Some("unit"), "100C to F", ExpectedOutcome::Ok("unit")),
        case("0 kelvin to celsius", Some("unit"), "absolute zero to C", ExpectedOutcome::Ok("unit")),
        case("273.15 kelvin to celsius", Some("unit"), "273.15K to C", ExpectedOutcome::Ok("unit")),
        case("100 celsius to kelvin", Some("unit"), "100C to K", ExpectedOutcome::Ok("unit")),
        case("-40 celsius to fahrenheit", Some("unit"), "-40C to F (same value)", ExpectedOutcome::Ok("unit")),
        case("98.6 fahrenheit to celsius", Some("unit"), "body temp F to C", ExpectedOutcome::Ok("unit")),
        case("72 fahrenheit to celsius", Some("unit"), "room temp F to C", ExpectedOutcome::Ok("unit")),
        case("350 fahrenheit to celsius", Some("unit"), "oven temp F to C", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Speed
        case("100 km/h to mph", Some("unit"), "km/h to mph", ExpectedOutcome::Ok("unit")),
        case("60 mph to km/h", Some("unit"), "mph to km/h", ExpectedOutcome::Ok("unit")),
        case("1 mach to km/h", Some("unit"), "mach to km/h", ExpectedOutcome::Ok("unit")),
        case("10 m/s to km/h", Some("unit"), "m/s to km/h", ExpectedOutcome::Ok("unit")),
        case("1 knots to km/h", Some("unit"), "knots to km/h", ExpectedOutcome::Ok("unit")),
        case("100 mph to m/s", Some("unit"), "mph to m/s", ExpectedOutcome::Ok("unit")),
        case("340 m/s to mach", Some("unit"), "speed of sound to mach", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Time
        case("1 hr to min", Some("unit"), "hour to minutes", ExpectedOutcome::Ok("unit")),
        case("1 day to hr", Some("unit"), "day to hours", ExpectedOutcome::Ok("unit")),
        case("1 week to day", Some("unit"), "week to days", ExpectedOutcome::Ok("unit")),
        case("1 year to day", Some("unit"), "year to days", ExpectedOutcome::Ok("unit")),
        case("1 decade to year", Some("unit"), "decade to years", ExpectedOutcome::Ok("unit")),
        case("1 century to year", Some("unit"), "century to years", ExpectedOutcome::Ok("unit")),
        case("60 s to min", Some("unit"), "seconds to minutes", ExpectedOutcome::Ok("unit")),
        case("1000 ms to s", Some("unit"), "milliseconds to seconds", ExpectedOutcome::Ok("unit")),
        case("1000000 ns to ms", Some("unit"), "nanoseconds to ms", ExpectedOutcome::Ok("unit")),
        case("1 month to day", Some("unit"), "month to days", ExpectedOutcome::Ok("unit")),
        case("24 hr to day", Some("unit"), "24 hours to days", ExpectedOutcome::Ok("unit")),
        case("365 day to year", Some("unit"), "365 days to years", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Data
        case("1 gb to mb", Some("unit"), "GB to MB", ExpectedOutcome::Ok("unit")),
        case("1 tb to gb", Some("unit"), "TB to GB", ExpectedOutcome::Ok("unit")),
        case("1 mb to kb", Some("unit"), "MB to KB", ExpectedOutcome::Ok("unit")),
        case("8 bit to byte", Some("unit"), "bits to byte", ExpectedOutcome::Ok("unit")),
        case("1 pb to tb", Some("unit"), "PB to TB", ExpectedOutcome::Ok("unit")),
        case("1 gib to mib", Some("unit"), "GiB to MiB", ExpectedOutcome::Ok("unit")),
        case("1 tib to gib", Some("unit"), "TiB to GiB", ExpectedOutcome::Ok("unit")),
        case("1 kib to byte", Some("unit"), "KiB to bytes", ExpectedOutcome::Ok("unit")),
        case("1024 mb to gb", Some("unit"), "1024 MB to GB", ExpectedOutcome::Ok("unit")),
        case("1 gb to byte", Some("unit"), "GB to bytes", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Pressure
        case("1 atm to psi", Some("unit"), "atmosphere to psi", ExpectedOutcome::Ok("unit")),
        case("1 bar to kPa", Some("unit"), "bar to kilopascals", ExpectedOutcome::Ok("unit")),
        case("760 mmHg to atm", Some("unit"), "mmHg to atmosphere", ExpectedOutcome::Ok("unit")),
        case("1 atm to torr", Some("unit"), "atm to torr", ExpectedOutcome::Ok("unit")),
        case("100 kPa to bar", Some("unit"), "kPa to bar", ExpectedOutcome::Ok("unit")),
        case("14.7 psi to atm", Some("unit"), "psi to atm", ExpectedOutcome::Ok("unit")),
        case("101325 Pa to atm", Some("unit"), "pascals to atm", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Energy
        case("1 kWh to J", Some("unit"), "kWh to joules", ExpectedOutcome::Ok("unit")),
        case("1 cal to J", Some("unit"), "calorie to joules", ExpectedOutcome::Ok("unit")),
        case("1000 cal to kcal", Some("unit"), "calories to kilocalories", ExpectedOutcome::Ok("unit")),
        case("1 BTU to J", Some("unit"), "BTU to joules", ExpectedOutcome::Ok("unit")),
        case("1 eV to J", Some("unit"), "electron volt to joules", ExpectedOutcome::Ok("unit")),
        case("1 kJ to cal", Some("unit"), "kJ to calories", ExpectedOutcome::Ok("unit")),
        case("1 Wh to J", Some("unit"), "watt-hour to joules", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Power
        case("1 kW to W", Some("unit"), "kilowatt to watts", ExpectedOutcome::Ok("unit")),
        case("1 hp to W", Some("unit"), "horsepower to watts", ExpectedOutcome::Ok("unit")),
        case("1 MW to kW", Some("unit"), "megawatt to kilowatts", ExpectedOutcome::Ok("unit")),
        case("1 GW to MW", Some("unit"), "gigawatt to megawatts", ExpectedOutcome::Ok("unit")),
        case("746 W to hp", Some("unit"), "watts to horsepower", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Angle
        case("180 deg to rad", Some("unit"), "degrees to radians", ExpectedOutcome::Ok("unit")),
        case("1 rad to deg", Some("unit"), "radians to degrees", ExpectedOutcome::Ok("unit")),
        case("360 deg to revolution", Some("unit"), "degrees to revolution", ExpectedOutcome::Ok("unit")),
        case("1 revolution to deg", Some("unit"), "revolution to degrees", ExpectedOutcome::Ok("unit")),
        case("100 grad to deg", Some("unit"), "gradians to degrees", ExpectedOutcome::Ok("unit")),
        case("60 arcmin to deg", Some("unit"), "arcminutes to degrees", ExpectedOutcome::Ok("unit")),
        case("3600 arcsec to deg", Some("unit"), "arcseconds to degrees", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Frequency
        case("1 GHz to MHz", Some("unit"), "GHz to MHz", ExpectedOutcome::Ok("unit")),
        case("1 MHz to kHz", Some("unit"), "MHz to kHz", ExpectedOutcome::Ok("unit")),
        case("1 kHz to Hz", Some("unit"), "kHz to Hz", ExpectedOutcome::Ok("unit")),
        case("1 THz to GHz", Some("unit"), "THz to GHz", ExpectedOutcome::Ok("unit")),
        case("60 RPM to Hz", Some("unit"), "RPM to Hz", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Electric Current
        case("1 A to mA", Some("unit"), "amperes to milliamps", ExpectedOutcome::Ok("unit")),
        case("1 kA to A", Some("unit"), "kiloamperes to amps", ExpectedOutcome::Ok("unit")),
        case("1000 mA to A", Some("unit"), "milliamps to amps", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Voltage
        case("1 kV to V", Some("unit"), "kilovolts to volts", ExpectedOutcome::Ok("unit")),
        case("1000 mV to V", Some("unit"), "millivolts to volts", ExpectedOutcome::Ok("unit")),
        case("1 V to mV", Some("unit"), "volts to millivolts", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Fuel Economy
        case("30 mpg to km/l", Some("unit"), "mpg to km/l", ExpectedOutcome::Ok("unit")),
        case("10 km/l to mpg", Some("unit"), "km/l to mpg", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSIONS - Natural Language Variations
        case("convert 5 km to miles", Some("unit"), "natural: convert X to Y", ExpectedOutcome::Ok("unit")),
        case("10 kilometers to miles", Some("unit"), "natural: full unit names", ExpectedOutcome::Ok("unit")),
        case("100 centimeters to meters", Some("unit"), "natural: centimeters full name", ExpectedOutcome::Ok("unit")),
        case("5 kilograms to pounds", Some("unit"), "natural: kilograms full name", ExpectedOutcome::Ok("unit")),
        case("1 liter to milliliters", Some("unit"), "natural: liter full name", ExpectedOutcome::Ok("unit")),
        case("60 miles per hour to km/h", Some("unit"), "natural: miles per hour", ExpectedOutcome::Ok("unit")),
        case("1 meter to feet", Some("unit"), "natural: meter to feet", ExpectedOutcome::Ok("unit")),
        case("1 foot to meter", Some("unit"), "natural: foot singular", ExpectedOutcome::Ok("unit")),
        case("2 feet to meters", Some("unit"), "natural: feet plural", ExpectedOutcome::Ok("unit")),
        case("1 inch to centimeters", Some("unit"), "natural: inch singular", ExpectedOutcome::Ok("unit")),
        case("12 inches to feet", Some("unit"), "natural: inches plural", ExpectedOutcome::Ok("unit")),
        case("1 pound to kilograms", Some("unit"), "natural: pound full", ExpectedOutcome::Ok("unit")),
        case("1 ounce to grams", Some("unit"), "natural: ounce full", ExpectedOutcome::Ok("unit")),
        case("1 mile to kilometers", Some("unit"), "natural: mile singular", ExpectedOutcome::Ok("unit")),
        case("5 miles to km", Some("unit"), "natural: miles plural", ExpectedOutcome::Ok("unit")),
        case("1 yard to meters", Some("unit"), "natural: yard full", ExpectedOutcome::Ok("unit")),
        case("1 gallon to liters", Some("unit"), "natural: gallon full", ExpectedOutcome::Ok("unit")),
        case("2 gallons to liters", Some("unit"), "natural: gallons plural", ExpectedOutcome::Ok("unit")),
        case("1 teaspoon to tablespoon", Some("unit"), "natural: teaspoon full", ExpectedOutcome::Ok("unit")),
        case("1 tablespoon to teaspoon", Some("unit"), "natural: tablespoon full", ExpectedOutcome::Ok("unit")),

        // UNIT CONVERSION - Edge Cases
        case("0 km to m", Some("unit"), "zero value conversion", ExpectedOutcome::Ok("unit")),
        case("0 celsius to fahrenheit", Some("unit"), "zero celsius to F", ExpectedOutcome::Ok("unit")),
        case("-273.15 celsius to kelvin", Some("unit"), "absolute zero C to K", ExpectedOutcome::Ok("unit")),
        case("1000000 bytes to mb", Some("unit"), "bytes to MB", ExpectedOutcome::Ok("unit")),
        case("0.001 km to m", Some("unit"), "tiny km to m", ExpectedOutcome::Ok("unit")),
        case("999999 m to km", Some("unit"), "large meters to km", ExpectedOutcome::Ok("unit")),
    ]
}

#[test]
fn imported_super_calculator_unit_cases() {
    block_on(run_cases("unit", cases()));
}
