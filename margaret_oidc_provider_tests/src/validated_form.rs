use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde_json::Value;
use validator::Validate;

use margaret_validation::validate::validate;
use margaret_validation::validation_result::ValidationResult;

/// # Panics
///
/// Panics when the parameters are not an object of strings.
#[must_use]
pub fn validated_form<TForm: DeserializeOwned + Validate>(
    parameters: &Value,
) -> ValidationResult<TForm> {
    validate(
        &parameters
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
            .collect::<HashMap<String, String>>(),
    )
}
