use serde_json::json;

use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::token_admission::TokenAdmission;
use margaret_token_introspection_tests::introspected_with::introspected_with;
use margaret_token_introspection_tests::repository_claims::RepositoryClaims;

#[tokio::test]
async fn refuses_while_the_introspection_endpoint_refuses_its_client() {
    assert!(matches!(
        introspected_with::<RepositoryClaims>(401, &json!({ "error": "invalid_client" })).await,
        TokenAdmission::Refused(ResponseContinuation::Done(response))
            if response.status() == 503
    ));
}
