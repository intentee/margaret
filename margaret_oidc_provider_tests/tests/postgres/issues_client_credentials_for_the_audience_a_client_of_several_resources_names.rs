use serde_json::json;

use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_oidc_provider_tests::jws_claims::jws_claims;
use margaret_oidc_provider_tests::ledger_credentials::LEDGER_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn issues_client_credentials_for_the_audience_a_client_of_several_resources_names() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/token",
            &LEDGER_CREDENTIALS,
            &json!({"audience": "reports", "grant_type": "client_credentials"}),
        )
        .await;
    let CompactJwsParsing::Parsed(access_token) = CompactJws::parse(answer.member("access_token"))
    else {
        panic!("the access token is a compact jws");
    };

    assert_eq!(answer.status, 200);
    assert_eq!(jws_claims(&access_token)["aud"], "reports");

    fixture.stop().await;
}
