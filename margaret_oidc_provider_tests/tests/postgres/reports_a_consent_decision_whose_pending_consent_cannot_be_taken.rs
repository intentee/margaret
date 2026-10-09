use std::sync::Arc;

use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_handler_error::handler_error::HandlerError;
use margaret_http::body_limit::BodyLimit;
use margaret_http::handles_limited_content::HandlesLimitedContent;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::consent_handler::ConsentHandler;
use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider_tests::form_post_request::form_post_request;
use margaret_oidc_provider_tests::owned_body::owned_body;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_session_cookies::signed_in_session_cookies;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_consent_decision_whose_pending_consent_cannot_be_taken() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::ConsentRequired(consent) =
        fixture.authorized(&spa_parameters()).await
    else {
        panic!("the end user is asked for consent");
    };
    let cookies = signed_in_session_cookies(&fixture.sessions).await;

    fixture
        .storage
        .administration
        .revoke(
            TablePrivilege::Delete,
            TableNamespace::Framework,
            "pending_authorizations",
        )
        .await;

    let answered =
        ConsentHandler::create(Arc::clone(&fixture.consent), Arc::clone(&fixture.sessions))
            .handle(
                &form_post_request(&cookies),
                owned_body(format!("decision=approve&id={}", consent.id)),
                BodyLimit::new(1_024),
            )
            .await;

    fixture.stop().await;

    assert!(matches!(
        answered,
        Err(HandlerError::Consumer { source })
            if matches!(
                source.downcast_ref(),
                Some(ProviderError::AuthorizationGrants(
                    AuthorizationGrantsError::TakePendingAuthorization(_)
                ))
            )
    ));
}
