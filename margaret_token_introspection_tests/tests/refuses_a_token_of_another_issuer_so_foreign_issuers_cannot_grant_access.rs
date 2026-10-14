use serde_json::json;

use margaret_http::token_admission::TokenAdmission;
use margaret_token_introspection_tests::introspected_with::introspected_with;
use margaret_token_introspection_tests::repository_claims::RepositoryClaims;

#[tokio::test]
async fn refuses_a_token_of_another_issuer_so_foreign_issuers_cannot_grant_access() {
    assert!(matches!(
        introspected_with::<RepositoryClaims>(200, &json!({ "active": true, "aud": "margaret", "iss": "https://attacker.example", "repository": "intentee/margaret" })).await,
        TokenAdmission::Refused(response)
            if response.status() == 401
    ));
}
