use serde_json::json;

use margaret_oidc_provider_tests::ci_issuer::CiIssuer;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_a_subject_token_of_a_type_its_exchanger_does_not_admit() {
    let ci = CiIssuer::publishing_keys();
    let fixture = ProviderFixture::start(vec![ci.exchanger.clone()]).await;
    let answer = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &json!({
                "grant_type": "urn:ietf:params:oauth:grant-type:token-exchange",
                "subject_token": ci.token("intentee/margaret"),
                "subject_token_type": "urn:ietf:params:oauth:token-type:access_token",
            }),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_request");
    assert_eq!(
        answer.body["error_description"],
        "the subject token type does not match the profile of its exchanger"
    );

    fixture.stop().await;
}
