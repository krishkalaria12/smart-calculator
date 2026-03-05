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

// Helper to create UnitDef
const fn u(factor: f64, name: &'static str, offset: f64) -> UnitDef {
    UnitDef {
        factor,
        offset,
        name,
    }
}

/// The global unit index for fast alias lookups
pub static UNIT_INDEX: LazyLock<HashMap<String, UnitLookupEntry>> = LazyLock::new(|| {
    let map = HashMap::new();
    let pinned = map.pin();

    // Macro to avoid repetitive boilerplate for each category
    macro_rules! insert_units {
        ($cat:expr, [ $( ($alias:expr, $factor:expr, $name:expr $(, $offset:expr)? ) ),* $(,)? ]) => {
            $(
                let offset = 0.0 $( + $offset )?;
                pinned.insert($alias.to_string(), UnitLookupEntry {
                    category_name: $cat,
                    def: u($factor, $name, offset),
                });
            )*
        };
    }

    // --- LENGTH ---
    insert_units!(
        "length",
        [
            ("nm", 1e-9, "nanometer"),
            ("nanometer", 1e-9, "nanometer"),
            ("nanometers", 1e-9, "nanometer"),
            ("mm", 0.001, "millimeter"),
            ("millimeter", 0.001, "millimeter"),
            ("m", 1.0, "meter"),
            ("meter", 1.0, "meter"),
            ("km", 1000.0, "kilometer"),
            ("in", 0.0254, "inch"),
            ("inch", 0.0254, "inch"),
            ("ft", 0.3048, "foot"),
            ("foot", 0.3048, "foot"),
            ("feet", 0.3048, "foot"),
            ("mi", 1609.344, "mile"),
            ("mile", 1609.344, "mile"),
        ]
    );

    // --- MASS ---
    insert_units!(
        "mass",
        [
            ("mg", 1e-6, "milligram"),
            ("g", 0.001, "gram"),
            ("kg", 1.0, "kilogram"),
            ("kilogram", 1.0, "kilogram"),
            ("lb", 0.45359237, "pound"),
            ("lbs", 0.45359237, "pound"),
            ("oz", 0.028349523125, "ounce"),
        ]
    );

    // --- TEMPERATURE ---
    insert_units!(
        "temperature",
        [
            ("k", 1.0, "kelvin", 0.0),
            ("kelvin", 1.0, "kelvin", 0.0),
            ("c", 1.0, "celsius", 273.15),
            ("celsius", 1.0, "celsius", 273.15),
            ("f", 5.0 / 9.0, "fahrenheit", 459.67),
            ("fahrenheit", 5.0 / 9.0, "fahrenheit", 459.67),
        ]
    );

    // --- DATA SIZE ---
    insert_units!(
        "data",
        [
            ("b", 1.0, "byte"),
            ("byte", 1.0, "byte"),
            ("kb", 1000.0, "kilobyte"),
            ("mb", 1e6, "megabyte"),
            ("gb", 1e9, "gigabyte"),
            ("tb", 1e12, "terabyte"),
            ("kib", 1024.0, "kibibyte"),
            ("mib", 1048576.0, "mebibyte"),
        ]
    );

    // --- TIME ---
    insert_units!(
        "time",
        [
            ("s", 1.0, "second"),
            ("sec", 1.0, "second"),
            ("min", 60.0, "minute"),
            ("h", 3600.0, "hour"),
            ("hr", 3600.0, "hour"),
            ("d", 86400.0, "day"),
        ]
    );

    // --- ANGLE ---
    insert_units!(
        "angle",
        [
            ("rad", 1.0, "radian"),
            ("deg", std::f64::consts::PI / 180.0, "degree"),
            ("°", std::f64::consts::PI / 180.0, "degree"),
        ]
    );

    drop(pinned);
    map
});

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
        .replace('²', "2")
        .replace('³', "3")
}
