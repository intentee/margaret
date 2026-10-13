use margaret_http::body_limit::BodyLimit;
use margaret_http::handles_limited_content::HandlesLimitedContent;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_oidc_provider_tests::fixture_authorization_handler::fixture_authorization_handler;
use margaret_oidc_provider_tests::fixture_consent_view::FixtureConsentView;
use margaret_oidc_provider_tests::form_encoded::form_encoded;
use margaret_oidc_provider_tests::form_post_request::form_post_request;
use margaret_oidc_provider_tests::owned_body::owned_body;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;

#[tokio::test]
async fn asks_an_anonymous_end_user_posting_an_authorization_to_sign_in() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answered = fixture_authorization_handler(&fixture, FixtureConsentView)
        .handle(
            &form_post_request(&[]),
            owned_body(form_encoded(&spa_parameters())),
            BodyLimit::new(16_384),
        )
        .await;

    fixture.stop().await;

    assert!(matches!(
        answered,
        Ok(ResponseContinuation::Done(response)) if response.status() == 401
    ));
}
