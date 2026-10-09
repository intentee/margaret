use form_urlencoded::Serializer;
use serde_json::Value;

use crate::string_parameters::string_parameters;

#[must_use]
pub fn form_encoded(parameters: &Value) -> String {
    Serializer::new(String::new())
        .extend_pairs(string_parameters(parameters))
        .finish()
}
