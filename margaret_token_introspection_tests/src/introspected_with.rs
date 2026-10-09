use serde::de::DeserializeOwned;
use serde_json::Value;

use margaret_http::token_admission::TokenAdmission;
use margaret_token_introspection::introspected_token::IntrospectedToken;

use crate::introspected_with_body::introspected_with_body;

pub async fn introspected_with<TClaims: DeserializeOwned>(
    status: u16,
    response: &Value,
) -> TokenAdmission<IntrospectedToken<TClaims>> {
    introspected_with_body(status, response.to_string().into_bytes()).await
}
