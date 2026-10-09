use serde_json::Value;
use serde_json::json;

use crate::answer::Answer;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::provider_fixture::ProviderFixture;

pub async fn refreshed_tokens(
    fixture: &ProviderFixture,
    refresh_token: &Value,
    scope: Option<&str>,
) -> Answer {
    let mut form = json!({
        "grant_type": "refresh_token",
        "refresh_token": refresh_token,
    });

    if let Some(scope) = scope {
        form["scope"] = Value::String(scope.to_string());
    }

    fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &form)
        .await
}
