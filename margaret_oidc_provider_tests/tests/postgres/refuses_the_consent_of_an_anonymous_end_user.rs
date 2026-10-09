use std::sync::Arc;

use margaret_http::body_limit::BodyLimit;
use margaret_http::handles_limited_content::HandlesLimitedContent;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_oidc_provider::consent_handler::ConsentHandler;
use margaret_oidc_provider_tests::form_post_request::form_post_request;
use margaret_oidc_provider_tests::owned_body::owned_body;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_the_consent_of_an_anonymous_end_user() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answered =
        ConsentHandler::create(Arc::clone(&fixture.consent), Arc::clone(&fixture.sessions))
            .handle(
                &form_post_request(&[]),
                owned_body("decision=approve&id=00000000-0000-0000-0000-000000000000".to_string()),
                BodyLimit::new(1_024),
            )
            .await;

    fixture.stop().await;

    assert!(matches!(
        answered,
        Ok(ResponseContinuation::Done(response)) if response.status() == 401
    ));
}
