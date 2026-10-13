use serde_json::json;

use margaret_oidc_provider_tests::ci_issuer::CiIssuer;
use margaret_oidc_provider_tests::exchange_request::exchange_request;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_a_subject_token_addressed_to_two_exchangers() {
    let ci = CiIssuer::publishing_keys();
    let deploy = CiIssuer::publishing_keys_for("https://deploy.localhost");
    let fixture =
        ProviderFixture::start(vec![ci.exchanger.clone(), deploy.exchanger.clone()]).await;
    let answer = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &exchange_request(&ci.token_addressed_to(
                &json!(["https://localhost", "https://deploy.localhost"]),
                "intentee/margaret",
            )),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_request");
    assert_eq!(
        answer.body["error_description"],
        "the subject token is addressed to more than one exchanger"
    );

    fixture.stop().await;
}
