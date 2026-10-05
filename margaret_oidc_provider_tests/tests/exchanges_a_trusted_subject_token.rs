use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_oidc_provider_tests::ci_issuer::CiIssuer;
use margaret_oidc_provider_tests::ci_subject::CI_SUBJECT;
use margaret_oidc_provider_tests::exchange_request::exchange_request;
use margaret_oidc_provider_tests::jws_claims::jws_claims;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

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
    let CompactJwsParsing::Parsed(access_token) = CompactJws::parse(answer.member("access_token"))
    else {
        panic!("the access token is a compact jws");
    };
    let access_claims = jws_claims(&access_token);

    assert_eq!(answer.status, 200);
    assert_eq!(
        answer.body["issued_token_type"],
        "urn:ietf:params:oauth:token-type:access_token"
    );
    assert_eq!(answer.body["scope"], "profile");
    assert_eq!(access_claims["aud"], "artifacts");
    assert_eq!(access_claims["sub"], CI_SUBJECT.to_string());

    fixture.stop().await;
}
