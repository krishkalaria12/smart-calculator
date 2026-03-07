use std::collections::HashMap;
use std::f64::consts;
use std::sync::LazyLock;

pub type Number = f64;
pub type UnaryFn = fn(Number) -> Number;
pub type BinaryFn = fn(Number, Number) -> Number;

pub static CONSTANTS: LazyLock<HashMap<&'static str, Number>> = LazyLock::new(|| {
    HashMap::from([
        ("pi", consts::PI),
        ("π", consts::PI),
        ("e", consts::E),
        ("tau", consts::TAU),
        ("τ", consts::TAU),
        ("phi", (1.0 + 5.0_f64.sqrt()) / 2.0),
        ("φ", (1.0 + 5.0_f64.sqrt()) / 2.0),
        ("inf", Number::INFINITY),
        ("infinity", Number::INFINITY),
    ])
});

pub static FUNCTIONS: LazyLock<HashMap<&'static str, UnaryFn>> = LazyLock::new(|| {
    HashMap::from([
        ("sqrt", f64::sqrt as UnaryFn),
        ("cbrt", f64::cbrt as UnaryFn),
        ("abs", f64::abs as UnaryFn),
        ("ceil", f64::ceil as UnaryFn),
        ("floor", f64::floor as UnaryFn),
        ("round", f64::round as UnaryFn),
        ("trunc", f64::trunc as UnaryFn),
        ("sign", f64::signum as UnaryFn),
        ("log", f64::log10 as UnaryFn),
        ("log2", f64::log2 as UnaryFn),
        ("log10", f64::log10 as UnaryFn),
        ("ln", f64::ln as UnaryFn),
        ("exp", f64::exp as UnaryFn),
        ("sin", f64::sin as UnaryFn),
        ("cos", f64::cos as UnaryFn),
        ("tan", f64::tan as UnaryFn),
        ("asin", f64::asin as UnaryFn),
        ("acos", f64::acos as UnaryFn),
        ("atan", f64::atan as UnaryFn),
        ("sinh", f64::sinh as UnaryFn),
        ("cosh", f64::cosh as UnaryFn),
        ("tanh", f64::tanh as UnaryFn),
        ("asinh", f64::asinh as UnaryFn),
        ("acosh", f64::acosh as UnaryFn),
        ("atanh", f64::atanh as UnaryFn),
    ])
});

pub static FUNCTIONS2: LazyLock<HashMap<&'static str, BinaryFn>> = LazyLock::new(|| {
    HashMap::from([
        ("pow", f64::powf as BinaryFn),
        ("max", f64::max as BinaryFn),
        ("min", f64::min as BinaryFn),
        ("atan2", f64::atan2 as BinaryFn),
        ("mod", |a, b| ((a % b) + b) % b),
    ])
});
