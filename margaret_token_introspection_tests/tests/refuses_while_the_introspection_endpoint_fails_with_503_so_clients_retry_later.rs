use serde_json::json;

use margaret_http::token_admission::TokenAdmission;
use margaret_token_introspection_tests::introspected_with::introspected_with;
use margaret_token_introspection_tests::repository_claims::RepositoryClaims;

#[tokio::test]
async fn refuses_while_the_introspection_endpoint_fails_with_503_so_clients_retry_later() {
    assert!(matches!(
        introspected_with::<RepositoryClaims>(500, &json!({})).await,
        TokenAdmission::Refused(response)
            if response.status() == 503
    ));
}
