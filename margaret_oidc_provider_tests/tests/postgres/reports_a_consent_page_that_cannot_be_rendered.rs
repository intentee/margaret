use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_oidc_provider_tests::authorization_page_request::authorization_page_request;
use margaret_oidc_provider_tests::failing_consent_view::FailingConsentView;
use margaret_oidc_provider_tests::fixture_authorization_handler::fixture_authorization_handler;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_session_cookies::signed_in_session_cookies;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;

#[tokio::test]
async fn reports_a_consent_page_that_cannot_be_rendered() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let cookies = signed_in_session_cookies(&fixture.sessions).await;
    let answered = fixture_authorization_handler(&fixture, FailingConsentView)
        .handle(&authorization_page_request(&spa_parameters(), &cookies))
        .await;

    fixture.stop().await;

    assert!(matches!(
        answered,
        Err(HandlerError::Consumer { source }) if source.to_string() == "the consent page cannot be rendered"
    ));
}
