use serde_json::json;

use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_oidc_provider_tests::constants::SERVICE_CREDENTIALS;
use margaret_oidc_provider_tests::jws_claims::jws_claims;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn issues_a_client_credentials_access_token_for_the_client_itself_without_a_refresh_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/token",
            &SERVICE_CREDENTIALS,
            &json!({"grant_type": "client_credentials"}),
        )
        .await;
    let CompactJwsParsing::Parsed(access_token) = CompactJws::parse(answer.member("access_token"))
    else {
        panic!("the access token is a compact jws");
    };
    let access_claims = jws_claims(&access_token);

    assert_eq!(answer.status, 200);
    assert!(answer.body.get("refresh_token").is_none());
    assert_eq!(answer.body["scope"], "artifacts:read");
    assert_eq!(access_claims["aud"], "artifacts");
    assert_eq!(access_claims["client_id"], "service");
    assert_eq!(access_claims["sub"], "service");

    fixture.stop().await;
}
