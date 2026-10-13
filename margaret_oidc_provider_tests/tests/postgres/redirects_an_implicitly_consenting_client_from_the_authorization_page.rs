use margaret_http::head_handler::HeadHandler;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http_tests::redirection::Redirection;
use margaret_oidc_provider_tests::authorization_page_request::authorization_page_request;
use margaret_oidc_provider_tests::fixture_authorization_handler::fixture_authorization_handler;
use margaret_oidc_provider_tests::fixture_consent_view::FixtureConsentView;
use margaret_oidc_provider_tests::portal_callback::PORTAL_CALLBACK;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_session_cookies::signed_in_session_cookies;
use margaret_oidc_provider_tests::with_parameter::with_parameter;

#[tokio::test]
async fn redirects_an_implicitly_consenting_client_from_the_authorization_page() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let cookies = signed_in_session_cookies(&fixture.sessions).await;
    let answered = fixture_authorization_handler(&fixture, FixtureConsentView)
        .handle(&authorization_page_request(
            &with_parameter(portal_parameters(), "max_age", "3600"),
            &cookies,
        ))
        .await;

    fixture.stop().await;

    let Ok(ResponseContinuation::Done(response)) = answered else {
        panic!("the authorization page is answered");
    };
    let redirection = Redirection::of(&response);
    let mut callback = redirection.location.clone();

    callback.set_query(None);

    assert_eq!(response.status(), 303);
    assert_eq!(callback.as_str(), PORTAL_CALLBACK);
    assert_eq!(redirection.parameter("code").len(), 43);
}
