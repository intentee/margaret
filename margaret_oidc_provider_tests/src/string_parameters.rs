use std::collections::HashMap;

use serde_json::Value;

/// # Panics
///
/// Panics when the parameters are not an object of strings.
#[must_use]
pub fn string_parameters(parameters: &Value) -> HashMap<String, String> {
    parameters
        .as_object()
        .expect("the parameters are an object")
        .iter()
        .map(|(name, value)| {
            (
                name.clone(),
                value
                    .as_str()
                    .expect("every parameter is a string")
                    .to_string(),
            )
        })
        .collect()
}
