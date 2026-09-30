use crate::ci_issuer::CiIssuer;
use crate::ci_subject::CI_SUBJECT;
use crate::exchange_request::exchange_request;
use crate::jwt_parts::JwtParts;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn exchanges_a_trusted_subject_token() {
    let ci = CiIssuer::publishing_keys();
    let fixture = ProviderFixture::start(vec![ci.exchanger.clone()]).await;
    let answer = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &exchange_request(&ci.token("intentee/margaret")),
        )
        .await;
    let access_token = JwtParts::of(&answer.body["access_token"]);

    assert_eq!(answer.status, 200);
    assert_eq!(
        answer.body["issued_token_type"],
        "urn:ietf:params:oauth:token-type:access_token"
    );
    assert_eq!(answer.body["scope"], "profile");
    assert_eq!(access_token.payload["aud"], "artifacts");
    assert_eq!(access_token.payload["sub"], CI_SUBJECT.to_string());

    fixture.stop().await;
}
