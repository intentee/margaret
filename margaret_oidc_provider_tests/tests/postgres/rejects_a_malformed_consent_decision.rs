use std::sync::Arc;

use margaret_http::body_limit::BodyLimit;
use margaret_http::handles_limited_content::HandlesLimitedContent;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_oidc_provider::consent_handler::ConsentHandler;
use margaret_oidc_provider_tests::form_post_request::form_post_request;
use margaret_oidc_provider_tests::owned_body::owned_body;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_session_cookies::signed_in_session_cookies;

#[tokio::test]
async fn rejects_a_malformed_consent_decision() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let cookies = signed_in_session_cookies(&fixture.sessions).await;
    let answered =
        ConsentHandler::create(Arc::clone(&fixture.consent), Arc::clone(&fixture.sessions))
            .handle(
                &form_post_request(&cookies),
                owned_body("decision=maybe".to_string()),
                BodyLimit::new(1_024),
            )
            .await;

    fixture.stop().await;

    assert!(matches!(
        answered,
        Ok(ResponseContinuation::Done(response)) if response.status() == 422
    ));
}
