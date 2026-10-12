use serde_json::json;

use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::token_admission::TokenAdmission;
use margaret_token_introspection_tests::introspected_with::introspected_with;
use margaret_token_introspection_tests::repository_claims::RepositoryClaims;

#[tokio::test]
async fn refuses_an_active_but_expired_token_with_401_because_activity_does_not_override_exp() {
    assert!(matches!(
        introspected_with::<RepositoryClaims>(200, &json!({ "active": true, "aud": "margaret", "exp": 1, "repository": "intentee/margaret" })).await,
        TokenAdmission::Refused(ResponseContinuation::Done(response))
            if response.status() == 401
    ));
}
