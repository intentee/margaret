use http::header::CONTENT_SECURITY_POLICY;
use serde_json::json;

use margaret_http::head_handler::HeadHandler;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_oidc_provider_tests::authorization_page_request::authorization_page_request;
use margaret_oidc_provider_tests::fixture_authorization_handler::fixture_authorization_handler;
use margaret_oidc_provider_tests::fixture_consent_view::FixtureConsentView;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn rejects_a_malformed_authorization_page_request() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answered = fixture_authorization_handler(&fixture, FixtureConsentView)
        .handle(&authorization_page_request(
            &json!({ "client_id": "unknown" }),
            &[],
        ))
        .await;

    fixture.stop().await;

    assert!(matches!(
        answered,
        Ok(ResponseContinuation::Done(response))
            if response.status() == 400
                && response.header_value(&CONTENT_SECURITY_POLICY) == Some("frame-ancestors 'none'")
    ));
}
