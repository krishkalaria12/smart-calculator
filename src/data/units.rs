use papaya::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy)]
pub struct UnitDef {
    pub factor: f64,
    pub offset: f64,
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct UnitLookupEntry {
    pub category_name: &'static str,
    pub def: UnitDef,
}

const DEGREE: &str = "\u{00B0}";
const fn u(factor: f64, name: &'static str, offset: f64) -> UnitDef {
    UnitDef {
        factor,
        offset,
        name,
    }
}

pub static UNIT_INDEX: LazyLock<HashMap<String, UnitLookupEntry>> = LazyLock::new(|| {
    let map = HashMap::new();
    let pinned = map.pin();

    macro_rules! insert_units {
        ($cat:expr, [ $( ($alias:expr, $factor:expr, $name:expr $(, $offset:expr)? ) ),* $(,)? ]) => {
            $(
                let offset = 0.0 $( + $offset )?;
                pinned.insert(
                    $alias.to_string(),
                    UnitLookupEntry {
                        category_name: $cat,
                        def: u($factor, $name, offset),
                    },
                );
            )*
        };
    }

    insert_units!(
        "length",
        [
            ("nm", 1e-9, "nanometer"),
            ("nanometer", 1e-9, "nanometer"),
            ("nanometers", 1e-9, "nanometer"),
            ("um", 1e-6, "micrometer"),
            ("micrometer", 1e-6, "micrometer"),
            ("micrometers", 1e-6, "micrometer"),
            ("micron", 1e-6, "micrometer"),
            ("microns", 1e-6, "micrometer"),
            ("mm", 0.001, "millimeter"),
            ("millimeter", 0.001, "millimeter"),
            ("millimeters", 0.001, "millimeter"),
            ("millimetre", 0.001, "millimeter"),
            ("millimetres", 0.001, "millimeter"),
            ("cm", 0.01, "centimeter"),
            ("centimeter", 0.01, "centimeter"),
            ("centimeters", 0.01, "centimeter"),
            ("centimetre", 0.01, "centimeter"),
            ("centimetres", 0.01, "centimeter"),
            ("dm", 0.1, "decimeter"),
            ("decimeter", 0.1, "decimeter"),
            ("decimeters", 0.1, "decimeter"),
            ("m", 1.0, "meter"),
            ("meter", 1.0, "meter"),
            ("meters", 1.0, "meter"),
            ("metre", 1.0, "meter"),
            ("metres", 1.0, "meter"),
            ("km", 1000.0, "kilometer"),
            ("kilometer", 1000.0, "kilometer"),
            ("kilometers", 1000.0, "kilometer"),
            ("kilometre", 1000.0, "kilometer"),
            ("kilometres", 1000.0, "kilometer"),
            ("in", 0.0254, "inch"),
            ("inch", 0.0254, "inch"),
            ("inches", 0.0254, "inch"),
            ("\"", 0.0254, "inch"),
            ("ft", 0.3048, "foot"),
            ("foot", 0.3048, "foot"),
            ("feet", 0.3048, "foot"),
            ("'", 0.3048, "foot"),
            ("yd", 0.9144, "yard"),
            ("yard", 0.9144, "yard"),
            ("yards", 0.9144, "yard"),
            ("mi", 1609.344, "mile"),
            ("mile", 1609.344, "mile"),
            ("miles", 1609.344, "mile"),
            ("nmi", 1852.0, "nautical mile"),
            ("nautical mile", 1852.0, "nautical mile"),
            ("nautical miles", 1852.0, "nautical mile"),
            ("au", 1.495978707e11, "astronomical unit"),
            ("astronomical unit", 1.495978707e11, "astronomical unit"),
            ("ly", 9.4607e15, "light year"),
            ("light year", 9.4607e15, "light year"),
            ("light years", 9.4607e15, "light year"),
            ("lightyear", 9.4607e15, "light year"),
            ("lightyears", 9.4607e15, "light year"),
            ("pc", 3.0857e16, "parsec"),
            ("parsec", 3.0857e16, "parsec"),
            ("parsecs", 3.0857e16, "parsec")
        ]
    );

    insert_units!(
        "mass",
        [
            ("ug", 1e-9, "microgram"),
            ("microgram", 1e-9, "microgram"),
            ("micrograms", 1e-9, "microgram"),
            ("mcg", 1e-9, "microgram"),
            ("mg", 1e-6, "milligram"),
            ("milligram", 1e-6, "milligram"),
            ("milligrams", 1e-6, "milligram"),
            ("g", 0.001, "gram"),
            ("gram", 0.001, "gram"),
            ("grams", 0.001, "gram"),
            ("kg", 1.0, "kilogram"),
            ("kilogram", 1.0, "kilogram"),
            ("kilograms", 1.0, "kilogram"),
            ("kilo", 1.0, "kilogram"),
            ("kilos", 1.0, "kilogram"),
            ("tonne", 1000.0, "metric ton"),
            ("tonnes", 1000.0, "metric ton"),
            ("metric ton", 1000.0, "metric ton"),
            ("metric tons", 1000.0, "metric ton"),
            ("t", 1000.0, "metric ton"),
            ("oz", 0.028349523125, "ounce"),
            ("ounce", 0.028349523125, "ounce"),
            ("ounces", 0.028349523125, "ounce"),
            ("lb", 0.45359237, "pound"),
            ("lbs", 0.45359237, "pound"),
            ("pound", 0.45359237, "pound"),
            ("pounds", 0.45359237, "pound"),
            ("st", 6.35029318, "stone"),
            ("stone", 6.35029318, "stone"),
            ("stones", 6.35029318, "stone"),
            ("short ton", 907.18474, "short ton"),
            ("short tons", 907.18474, "short ton"),
            ("long ton", 1016.0469088, "long ton"),
            ("long tons", 1016.0469088, "long ton"),
            ("grain", 6.479891e-5, "grain"),
            ("grains", 6.479891e-5, "grain"),
            ("gr", 6.479891e-5, "grain"),
            ("carat", 0.0002, "carat"),
            ("carats", 0.0002, "carat"),
            ("ct", 0.0002, "carat")
        ]
    );

    insert_units!(
        "volume",
        [
            ("ml", 0.001, "milliliter"),
            ("milliliter", 0.001, "milliliter"),
            ("milliliters", 0.001, "milliliter"),
            ("millilitre", 0.001, "milliliter"),
            ("millilitres", 0.001, "milliliter"),
            ("cl", 0.01, "centiliter"),
            ("centiliter", 0.01, "centiliter"),
            ("centiliters", 0.01, "centiliter"),
            ("dl", 0.1, "deciliter"),
            ("deciliter", 0.1, "deciliter"),
            ("deciliters", 0.1, "deciliter"),
            ("l", 1.0, "liter"),
            ("liter", 1.0, "liter"),
            ("liters", 1.0, "liter"),
            ("litre", 1.0, "liter"),
            ("litres", 1.0, "liter"),
            ("kl", 1000.0, "kiloliter"),
            ("kiloliter", 1000.0, "kiloliter"),
            ("kiloliters", 1000.0, "kiloliter"),
            ("m3", 1000.0, "cubic meter"),
            ("cubic meter", 1000.0, "cubic meter"),
            ("cubic meters", 1000.0, "cubic meter"),
            ("cubic metre", 1000.0, "cubic meter"),
            ("cm3", 0.001, "cubic centimeter"),
            ("cubic centimeter", 0.001, "cubic centimeter"),
            ("cubic centimeters", 0.001, "cubic centimeter"),
            ("cc", 0.001, "cubic centimeter"),
            ("tsp", 0.00492892, "teaspoon"),
            ("teaspoon", 0.00492892, "teaspoon"),
            ("teaspoons", 0.00492892, "teaspoon"),
            ("tbsp", 0.0147868, "tablespoon"),
            ("tablespoon", 0.0147868, "tablespoon"),
            ("tablespoons", 0.0147868, "tablespoon"),
            ("fl oz", 0.0295735, "fluid ounce"),
            ("fluid ounce", 0.0295735, "fluid ounce"),
            ("fluid ounces", 0.0295735, "fluid ounce"),
            ("floz", 0.0295735, "fluid ounce"),
            ("cup", 0.236588, "cup"),
            ("cups", 0.236588, "cup"),
            ("pt", 0.473176, "pint"),
            ("pint", 0.473176, "pint"),
            ("pints", 0.473176, "pint"),
            ("qt", 0.946353, "quart"),
            ("quart", 0.946353, "quart"),
            ("quarts", 0.946353, "quart"),
            ("gal", 3.78541, "gallon"),
            ("gallon", 3.78541, "gallon"),
            ("gallons", 3.78541, "gallon"),
            ("imp gal", 4.54609, "imperial gallon"),
            ("imperial gallon", 4.54609, "imperial gallon"),
            ("imperial gallons", 4.54609, "imperial gallon"),
            ("imp pt", 0.568261, "imperial pint"),
            ("imperial pint", 0.568261, "imperial pint"),
            ("bbl", 158.987, "barrel"),
            ("barrel", 158.987, "barrel"),
            ("barrels", 158.987, "barrel")
        ]
    );

    insert_units!(
        "area",
        [
            ("mm2", 1e-6, "square millimeter"),
            ("sq mm", 1e-6, "square millimeter"),
            ("square millimeter", 1e-6, "square millimeter"),
            ("square millimeters", 1e-6, "square millimeter"),
            ("cm2", 1e-4, "square centimeter"),
            ("sq cm", 1e-4, "square centimeter"),
            ("square centimeter", 1e-4, "square centimeter"),
            ("square centimeters", 1e-4, "square centimeter"),
            ("m2", 1.0, "square meter"),
            ("sq m", 1.0, "square meter"),
            ("square meter", 1.0, "square meter"),
            ("square meters", 1.0, "square meter"),
            ("square metre", 1.0, "square meter"),
            ("km2", 1e6, "square kilometer"),
            ("sq km", 1e6, "square kilometer"),
            ("square kilometer", 1e6, "square kilometer"),
            ("square kilometers", 1e6, "square kilometer"),
            ("in2", 0.00064516, "square inch"),
            ("sq in", 0.00064516, "square inch"),
            ("square inch", 0.00064516, "square inch"),
            ("square inches", 0.00064516, "square inch"),
            ("ft2", 0.092903, "square foot"),
            ("sq ft", 0.092903, "square foot"),
            ("square foot", 0.092903, "square foot"),
            ("square feet", 0.092903, "square foot"),
            ("sqft", 0.092903, "square foot"),
            ("yd2", 0.836127, "square yard"),
            ("sq yd", 0.836127, "square yard"),
            ("square yard", 0.836127, "square yard"),
            ("square yards", 0.836127, "square yard"),
            ("mi2", 2589988.11, "square mile"),
            ("sq mi", 2589988.11, "square mile"),
            ("square mile", 2589988.11, "square mile"),
            ("square miles", 2589988.11, "square mile"),
            ("acre", 4046.8564224, "acre"),
            ("acres", 4046.8564224, "acre"),
            ("ac", 4046.8564224, "acre"),
            ("hectare", 10000.0, "hectare"),
            ("hectares", 10000.0, "hectare"),
            ("ha", 10000.0, "hectare")
        ]
    );

    insert_units!(
        "temperature",
        [
            ("k", 1.0, "kelvin", 0.0),
            ("kelvin", 1.0, "kelvin", 0.0),
            ("c", 1.0, "celsius", 273.15),
            ("celsius", 1.0, "celsius", 273.15),
            ("\u{00B0}c", 1.0, "celsius", 273.15),
            ("centigrade", 1.0, "celsius", 273.15),
            ("f", 5.0 / 9.0, "fahrenheit", 459.67),
            ("fahrenheit", 5.0 / 9.0, "fahrenheit", 459.67),
            ("\u{00B0}f", 5.0 / 9.0, "fahrenheit", 459.67)
        ]
    );

    insert_units!(
        "speed",
        [
            ("mps", 1.0, "meter per second"),
            ("m/s", 1.0, "meter per second"),
            ("meter per second", 1.0, "meter per second"),
            ("meters per second", 1.0, "meter per second"),
            ("km/h", 1.0 / 3.6, "kilometer per hour"),
            ("kmh", 1.0 / 3.6, "kilometer per hour"),
            ("kph", 1.0 / 3.6, "kilometer per hour"),
            ("kmph", 1.0 / 3.6, "kilometer per hour"),
            ("kilometer per hour", 1.0 / 3.6, "kilometer per hour"),
            ("kilometers per hour", 1.0 / 3.6, "kilometer per hour"),
            ("mph", 0.44704, "mile per hour"),
            ("mile per hour", 0.44704, "mile per hour"),
            ("miles per hour", 0.44704, "mile per hour"),
            ("fps", 0.3048, "foot per second"),
            ("ft/s", 0.3048, "foot per second"),
            ("feet per second", 0.3048, "foot per second"),
            ("knot", 0.514444, "knot"),
            ("knots", 0.514444, "knot"),
            ("kn", 0.514444, "knot"),
            ("kt", 0.514444, "knot"),
            ("mach", 343.0, "mach")
        ]
    );

    insert_units!(
        "time",
        [
            ("ns", 1e-9, "nanosecond"),
            ("nanosecond", 1e-9, "nanosecond"),
            ("nanoseconds", 1e-9, "nanosecond"),
            ("us", 1e-6, "microsecond"),
            ("microsecond", 1e-6, "microsecond"),
            ("microseconds", 1e-6, "microsecond"),
            ("\u{00B5}s", 1e-6, "microsecond"),
            ("\u{03BC}s", 1e-6, "microsecond"),
            ("ms", 0.001, "millisecond"),
            ("millisecond", 0.001, "millisecond"),
            ("milliseconds", 0.001, "millisecond"),
            ("s", 1.0, "second"),
            ("sec", 1.0, "second"),
            ("second", 1.0, "second"),
            ("seconds", 1.0, "second"),
            ("min", 60.0, "minute"),
            ("minute", 60.0, "minute"),
            ("minutes", 60.0, "minute"),
            ("mins", 60.0, "minute"),
            ("h", 3600.0, "hour"),
            ("hr", 3600.0, "hour"),
            ("hrs", 3600.0, "hour"),
            ("hour", 3600.0, "hour"),
            ("hours", 3600.0, "hour"),
            ("d", 86400.0, "day"),
            ("day", 86400.0, "day"),
            ("days", 86400.0, "day"),
            ("wk", 604800.0, "week"),
            ("week", 604800.0, "week"),
            ("weeks", 604800.0, "week"),
            ("mo", 2629746.0, "month"),
            ("month", 2629746.0, "month"),
            ("months", 2629746.0, "month"),
            ("yr", 31556952.0, "year"),
            ("year", 31556952.0, "year"),
            ("years", 31556952.0, "year"),
            ("decade", 315569520.0, "decade"),
            ("decades", 315569520.0, "decade"),
            ("century", 3155695200.0, "century"),
            ("centuries", 3155695200.0, "century")
        ]
    );

    insert_units!(
        "data",
        [
            ("b", 1.0, "byte"),
            ("byte", 1.0, "byte"),
            ("bytes", 1.0, "byte"),
            ("kb", 1000.0, "kilobyte"),
            ("kilobyte", 1000.0, "kilobyte"),
            ("kilobytes", 1000.0, "kilobyte"),
            ("mb", 1e6, "megabyte"),
            ("megabyte", 1e6, "megabyte"),
            ("megabytes", 1e6, "megabyte"),
            ("gb", 1e9, "gigabyte"),
            ("gigabyte", 1e9, "gigabyte"),
            ("gigabytes", 1e9, "gigabyte"),
            ("tb", 1e12, "terabyte"),
            ("terabyte", 1e12, "terabyte"),
            ("terabytes", 1e12, "terabyte"),
            ("pb", 1e15, "petabyte"),
            ("petabyte", 1e15, "petabyte"),
            ("petabytes", 1e15, "petabyte"),
            ("eb", 1e18, "exabyte"),
            ("exabyte", 1e18, "exabyte"),
            ("exabytes", 1e18, "exabyte"),
            ("kib", 1024.0, "kibibyte"),
            ("kibibyte", 1024.0, "kibibyte"),
            ("kibibytes", 1024.0, "kibibyte"),
            ("mib", 1048576.0, "mebibyte"),
            ("mebibyte", 1048576.0, "mebibyte"),
            ("mebibytes", 1048576.0, "mebibyte"),
            ("gib", 1073741824.0, "gibibyte"),
            ("gibibyte", 1073741824.0, "gibibyte"),
            ("gibibytes", 1073741824.0, "gibibyte"),
            ("tib", 1099511627776.0, "tebibyte"),
            ("tebibyte", 1099511627776.0, "tebibyte"),
            ("tebibytes", 1099511627776.0, "tebibyte"),
            ("bit", 0.125, "bit"),
            ("bits", 0.125, "bit"),
            ("kbit", 125.0, "kilobit"),
            ("kilobit", 125.0, "kilobit"),
            ("kilobits", 125.0, "kilobit"),
            ("mbit", 125000.0, "megabit"),
            ("megabit", 125000.0, "megabit"),
            ("megabits", 125000.0, "megabit"),
            ("gbit", 125000000.0, "gigabit"),
            ("gigabit", 125000000.0, "gigabit"),
            ("gigabits", 125000000.0, "gigabit")
        ]
    );

    insert_units!(
        "pressure",
        [
            ("pa", 1.0, "pascal"),
            ("pascal", 1.0, "pascal"),
            ("pascals", 1.0, "pascal"),
            ("hpa", 100.0, "hectopascal"),
            ("hectopascal", 100.0, "hectopascal"),
            ("kpa", 1000.0, "kilopascal"),
            ("kilopascal", 1000.0, "kilopascal"),
            ("mpa", 1e6, "megapascal"),
            ("megapascal", 1e6, "megapascal"),
            ("bar", 100000.0, "bar"),
            ("bars", 100000.0, "bar"),
            ("mbar", 100.0, "millibar"),
            ("millibar", 100.0, "millibar"),
            ("atm", 101325.0, "atmosphere"),
            ("atmosphere", 101325.0, "atmosphere"),
            ("atmospheres", 101325.0, "atmosphere"),
            ("psi", 6894.757, "psi"),
            ("torr", 133.322, "torr"),
            ("mmhg", 133.322, "mmHg"),
            ("inhg", 3386.39, "inHg")
        ]
    );

    insert_units!(
        "energy",
        [
            ("j", 1.0, "joule"),
            ("joule", 1.0, "joule"),
            ("joules", 1.0, "joule"),
            ("kj", 1000.0, "kilojoule"),
            ("kilojoule", 1000.0, "kilojoule"),
            ("kilojoules", 1000.0, "kilojoule"),
            ("mj", 1e6, "megajoule"),
            ("megajoule", 1e6, "megajoule"),
            ("cal", 4.184, "calorie"),
            ("calorie", 4.184, "calorie"),
            ("calories", 4.184, "calorie"),
            ("kcal", 4184.0, "kilocalorie"),
            ("kilocalorie", 4184.0, "kilocalorie"),
            ("kilocalories", 4184.0, "kilocalorie"),
            ("wh", 3600.0, "watt-hour"),
            ("watt-hour", 3600.0, "watt-hour"),
            ("watt hour", 3600.0, "watt-hour"),
            ("kwh", 3600000.0, "kilowatt-hour"),
            ("kilowatt-hour", 3600000.0, "kilowatt-hour"),
            ("kilowatt hour", 3600000.0, "kilowatt-hour"),
            ("btu", 1055.06, "BTU"),
            ("british thermal unit", 1055.06, "BTU"),
            ("ev", 1.602176634e-19, "electronvolt"),
            ("electronvolt", 1.602176634e-19, "electronvolt"),
            ("erg", 1e-7, "erg"),
            ("ergs", 1e-7, "erg"),
            ("therm", 1.055e8, "therm"),
            ("therms", 1.055e8, "therm")
        ]
    );

    insert_units!(
        "power",
        [
            ("w", 1.0, "watt"),
            ("watt", 1.0, "watt"),
            ("watts", 1.0, "watt"),
            ("mw", 0.001, "milliwatt"),
            ("milliwatt", 0.001, "milliwatt"),
            ("kw", 1000.0, "kilowatt"),
            ("kilowatt", 1000.0, "kilowatt"),
            ("kilowatts", 1000.0, "kilowatt"),
            ("megawatt", 1e6, "megawatt"),
            ("megawatts", 1e6, "megawatt"),
            ("gw", 1e9, "gigawatt"),
            ("gigawatt", 1e9, "gigawatt"),
            ("gigawatts", 1e9, "gigawatt"),
            ("hp", 745.7, "horsepower"),
            ("horsepower", 745.7, "horsepower"),
            ("metric hp", 735.499, "metric horsepower"),
            ("ps", 735.499, "metric horsepower"),
            ("btu/h", 0.293071, "BTU per hour")
        ]
    );

    insert_units!(
        "angle",
        [
            ("rad", 1.0, "radian"),
            ("radian", 1.0, "radian"),
            ("radians", 1.0, "radian"),
            ("deg", std::f64::consts::PI / 180.0, "degree"),
            ("degree", std::f64::consts::PI / 180.0, "degree"),
            ("degrees", std::f64::consts::PI / 180.0, "degree"),
            (DEGREE, std::f64::consts::PI / 180.0, "degree"),
            ("grad", std::f64::consts::PI / 200.0, "gradian"),
            ("gradian", std::f64::consts::PI / 200.0, "gradian"),
            ("gradians", std::f64::consts::PI / 200.0, "gradian"),
            ("gon", std::f64::consts::PI / 200.0, "gradian"),
            ("arcmin", std::f64::consts::PI / 10800.0, "arcminute"),
            ("arcminute", std::f64::consts::PI / 10800.0, "arcminute"),
            ("arcminutes", std::f64::consts::PI / 10800.0, "arcminute"),
            ("arcsec", std::f64::consts::PI / 648000.0, "arcsecond"),
            ("arcsecond", std::f64::consts::PI / 648000.0, "arcsecond"),
            ("arcseconds", std::f64::consts::PI / 648000.0, "arcsecond"),
            ("rev", 2.0 * std::f64::consts::PI, "revolution"),
            ("revolution", 2.0 * std::f64::consts::PI, "revolution"),
            ("revolutions", 2.0 * std::f64::consts::PI, "revolution"),
            ("turn", 2.0 * std::f64::consts::PI, "revolution"),
            ("turns", 2.0 * std::f64::consts::PI, "revolution")
        ]
    );

    insert_units!(
        "frequency",
        [
            ("hz", 1.0, "hertz"),
            ("hertz", 1.0, "hertz"),
            ("khz", 1000.0, "kilohertz"),
            ("kilohertz", 1000.0, "kilohertz"),
            ("mhz", 1e6, "megahertz"),
            ("megahertz", 1e6, "megahertz"),
            ("ghz", 1e9, "gigahertz"),
            ("gigahertz", 1e9, "gigahertz"),
            ("thz", 1e12, "terahertz"),
            ("terahertz", 1e12, "terahertz"),
            ("rpm", 1.0 / 60.0, "RPM")
        ]
    );

    insert_units!(
        "electric current",
        [
            ("a", 1.0, "ampere"),
            ("amp", 1.0, "ampere"),
            ("amps", 1.0, "ampere"),
            ("ampere", 1.0, "ampere"),
            ("amperes", 1.0, "ampere"),
            ("ma", 0.001, "milliampere"),
            ("milliamp", 0.001, "milliampere"),
            ("milliamps", 0.001, "milliampere"),
            ("milliampere", 0.001, "milliampere"),
            ("ka", 1000.0, "kiloampere"),
            ("kiloamp", 1000.0, "kiloampere"),
            ("kiloampere", 1000.0, "kiloampere"),
            ("\u{03BC}a", 1e-6, "microampere"),
            ("microamp", 1e-6, "microampere"),
            ("microampere", 1e-6, "microampere")
        ]
    );

    insert_units!(
        "voltage",
        [
            ("v", 1.0, "volt"),
            ("volt", 1.0, "volt"),
            ("volts", 1.0, "volt"),
            ("mv", 0.001, "millivolt"),
            ("millivolt", 0.001, "millivolt"),
            ("millivolts", 0.001, "millivolt"),
            ("kv", 1000.0, "kilovolt"),
            ("kilovolt", 1000.0, "kilovolt"),
            ("kilovolts", 1000.0, "kilovolt"),
            ("\u{03BC}v", 1e-6, "microvolt"),
            ("microvolt", 1e-6, "microvolt"),
            ("microvolts", 1e-6, "microvolt")
        ]
    );

    insert_units!(
        "fuel economy",
        [
            ("km/l", 1.0, "km per liter"),
            ("kmpl", 1.0, "km per liter"),
            ("km per liter", 1.0, "km per liter"),
            ("mpg", 0.425144, "miles per gallon"),
            ("miles per gallon", 0.425144, "miles per gallon"),
            ("mpg uk", 0.354006, "miles per gallon (UK)"),
            ("uk mpg", 0.354006, "miles per gallon (UK)")
        ]
    );

    drop(pinned);
    map
});

pub fn lookup_unit(token: &str) -> Option<UnitLookupEntry> {
    let normalized = normalize_token(token);
    UNIT_INDEX.pin().get(normalized.as_str()).copied()
}

pub fn convert_unit(
    value: f64,
    from: &UnitLookupEntry,
    to: &UnitLookupEntry,
) -> Result<f64, String> {
    if from.category_name != to.category_name {
        return Err(format!(
            "Cannot convert between {} and {}",
            from.category_name, to.category_name
        ));
    }

    let base_value = (value + from.def.offset) * from.def.factor;
    let target_value = (base_value / to.def.factor) - to.def.offset;

    Ok(target_value)
}

pub fn normalize_token(token: &str) -> String {
    token
        .to_lowercase()
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace('\u{00B2}', "2")
        .replace('\u{00B3}', "3")
        .replace('\u{00B5}', "\u{03BC}")
}
