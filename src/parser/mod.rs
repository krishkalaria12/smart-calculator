mod intent;
mod normalize;

pub use intent::detect_intent;
pub use normalize::{normalize_conversion_input, normalize_whitespace};
