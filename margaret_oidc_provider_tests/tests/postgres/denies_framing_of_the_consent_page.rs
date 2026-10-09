use http::header::CONTENT_SECURITY_POLICY;

use margaret_http::head_handler::HeadHandler;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_oidc_provider_tests::authorization_page_request::authorization_page_request;
use margaret_oidc_provider_tests::fixture_authorization_handler::fixture_authorization_handler;
use margaret_oidc_provider_tests::fixture_consent_view::FixtureConsentView;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_session_cookies::signed_in_session_cookies;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;

#[tokio::test]
async fn denies_framing_of_the_consent_page() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let cookies = signed_in_session_cookies(&fixture.sessions).await;
    let answered = fixture_authorization_handler(&fixture, FixtureConsentView)
        .handle(&authorization_page_request(&spa_parameters(), &cookies))
        .await;

    fixture.stop().await;

    assert!(matches!(
        answered,
        Ok(ResponseContinuation::Done(response))
            if response.status() == 200
                && response.header_value(&CONTENT_SECURITY_POLICY) == Some("frame-ancestors 'none'")
    ));
}
