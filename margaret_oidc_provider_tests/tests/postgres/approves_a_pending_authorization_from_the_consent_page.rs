use std::sync::Arc;

use margaret_http::body_limit::BodyLimit;
use margaret_http::handles_limited_content::HandlesLimitedContent;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http_tests::redirection::Redirection;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::consent_handler::ConsentHandler;
use margaret_oidc_provider_tests::form_post_request::form_post_request;
use margaret_oidc_provider_tests::owned_body::owned_body;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_session_cookies::signed_in_session_cookies;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;

#[tokio::test]
async fn approves_a_pending_authorization_from_the_consent_page() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::ConsentRequired(consent) =
        fixture.authorized(&spa_parameters()).await
    else {
        panic!("the end user is asked for consent");
    };
    let cookies = signed_in_session_cookies(&fixture.sessions).await;
    let answered =
        ConsentHandler::create(Arc::clone(&fixture.consent), Arc::clone(&fixture.sessions))
            .handle(
                &form_post_request(&cookies),
                owned_body(format!("decision=approve&id={}", consent.id)),
                BodyLimit::new(1_024),
            )
            .await;

    fixture.stop().await;

    let Ok(ResponseContinuation::Done(response)) = answered else {
        panic!("the consent is answered");
    };

    assert!(!Redirection::of(&response).parameter("code").is_empty());
}
