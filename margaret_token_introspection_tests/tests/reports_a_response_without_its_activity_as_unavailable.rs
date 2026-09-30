use serde_json::json;

use margaret_http::response_continuation::ResponseContinuation;
use margaret_token_introspection::introspection_admission::IntrospectionAdmission;
use margaret_token_introspection_tests::introspected_with::introspected_with;
use margaret_token_introspection_tests::repository_claims::RepositoryClaims;

#[tokio::test]
async fn reports_a_response_without_its_activity_as_unavailable() {
    assert!(matches!(
        introspected_with::<RepositoryClaims>(200, &json!({ "aud": "margaret", "repository": "intentee/margaret" })).await,
        IntrospectionAdmission::Refused(ResponseContinuation::Done(response))
            if response.status() == 503
    ));
}
