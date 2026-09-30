use serde::de::DeserializeOwned;
use serde_json::Value;

use margaret_token_introspection::introspection_admission::IntrospectionAdmission;

use crate::introspected_with_body::introspected_with_body;

/// # Panics
///
/// Panics when the introspection of a secret basic client fails to be sent.
pub async fn introspected_with<TClaims: DeserializeOwned>(
    status: u16,
    response: &Value,
) -> IntrospectionAdmission<TClaims> {
    introspected_with_body(status, response.to_string().into_bytes()).await
}
