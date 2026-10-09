use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::end_user_subject::END_USER_SUBJECT;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::jws_claims::jws_claims;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;

#[tokio::test]
async fn rotates_a_refresh_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let first =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["refresh_token"].clone();
    let rotated = refreshed_tokens(&fixture, &first, None).await;
    let CompactJwsParsing::Parsed(access_token) = CompactJws::parse(rotated.member("access_token"))
    else {
        panic!("the access token is a compact jws");
    };

    assert_eq!(rotated.status, 200);
    assert_ne!(rotated.body["refresh_token"], first);
    assert_eq!(rotated.body["scope"], "openid profile");
    assert_eq!(
        jws_claims(&access_token)["sub"],
        END_USER_SUBJECT.to_string()
    );
    assert_eq!(
        refreshed_tokens(&fixture, &rotated.body["refresh_token"], None)
            .await
            .status,
        200
    );

    fixture.stop().await;
}
