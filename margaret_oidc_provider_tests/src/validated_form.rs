use serde::de::DeserializeOwned;
use serde_json::Value;
use validator::Validate;

use margaret_validation::validate::validate;
use margaret_validation::validation_result::ValidationResult;

use crate::string_parameters::string_parameters;

#[must_use]
pub fn validated_form<TForm: DeserializeOwned + Validate>(
    parameters: &Value,
) -> ValidationResult<TForm> {
    validate(&string_parameters(parameters))
}
