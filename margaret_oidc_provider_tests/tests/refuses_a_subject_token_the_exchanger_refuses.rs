use crate::ci_issuer::CiIssuer;
use crate::exchange_request::exchange_request;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_a_subject_token_the_exchanger_refuses() {
    let ci = CiIssuer::publishing_keys();
    let fixture = ProviderFixture::start(vec![ci.exchanger.clone()]).await;
    let answer = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &exchange_request(&ci.token("someone/else")),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_grant");
    assert_eq!(
        answer.body["error_description"],
        "the exchanger refused the subject token"
    );

    fixture.stop().await;
}
