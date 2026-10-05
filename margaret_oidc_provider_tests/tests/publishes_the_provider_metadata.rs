use serde_json::json;

use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn publishes_the_provider_metadata() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture.get("/.well-known/openid-configuration").await;

    assert_eq!(answer.status, 200);
    assert_eq!(
        answer.body,
        json!({
            "authorization_endpoint": "https://localhost/authorize",
            "authorization_response_iss_parameter_supported": true,
            "code_challenge_methods_supported": ["S256"],
            "grant_types_supported": [
                "authorization_code",
                "client_credentials",
                "refresh_token",
                "urn:ietf:params:oauth:grant-type:token-exchange",
            ],
            "id_token_signing_alg_values_supported": ["ES256", "RS256"],
            "introspection_endpoint": "https://localhost/introspect",
            "introspection_endpoint_auth_methods_supported": ["client_secret_basic"],
            "issuer": "https://localhost",
            "jwks_uri": "https://localhost/jwks.json",
            "response_modes_supported": ["query"],
            "response_types_supported": ["code"],
            "revocation_endpoint": "https://localhost/revoke",
            "revocation_endpoint_auth_methods_supported": ["client_secret_basic", "none"],
            "scopes_supported": ["artifacts:read", "openid", "profile"],
            "subject_types_supported": ["public"],
            "token_endpoint": "https://localhost/token",
            "token_endpoint_auth_methods_supported": ["client_secret_basic", "none"],
            "userinfo_endpoint": "https://localhost/userinfo",
        })
    );

    fixture.stop().await;
}
