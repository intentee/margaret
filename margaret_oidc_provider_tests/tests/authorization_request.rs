use std::collections::HashMap;

use serde_json::Value;

use margaret_oidc_provider::authorization_request::AuthorizationRequest;
use margaret_validation::validate::validate;
use margaret_validation::validation_result::ValidationResult;

pub fn authorization_request(parameters: &Value) -> ValidationResult<AuthorizationRequest> {
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
