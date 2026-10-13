use serde_json::json;

use margaret_http::token_admission::TokenAdmission;
use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_token_introspection::introspected_token::IntrospectedToken;
use margaret_token_introspection_tests::introspected_with::introspected_with;
use margaret_token_introspection_tests::repository_claims::RepositoryClaims;

#[tokio::test]
async fn admits_an_active_token_for_our_audience() {
    let TokenAdmission::Admitted(IntrospectedToken {
        claims,
        client_id,
        scopes,
        subject,
        username,
    }) = introspected_with::<RepositoryClaims>(
        200,
        &json!({
            "active": true,
            "aud": ["other", "margaret"],
            "client_id": "uploader",
            "exp": 9_999_999_999_i64,
            "iss": "https://localhost",
            "nbf": 1,
            "repository": "intentee/margaret",
            "scope": "artifacts:read artifacts:write",
            "sub": "subject",
            "username": "ci",
        }),
    )
    .await
    else {
        panic!("the active token is admitted");
    };

    assert_eq!(
        claims,
        RepositoryClaims {
            repository: "intentee/margaret".to_string()
        }
    );
    assert_eq!(client_id.as_ref().map(ClientId::as_str), Some("uploader"));
    assert_eq!(
        scopes.map(|scopes| ScopeList { scopes }.to_string()),
        Some("artifacts:read artifacts:write".to_string())
    );
    assert_eq!(subject.as_deref(), Some("subject"));
    assert_eq!(username.as_deref(), Some("ci"));
}
