use chrono::TimeDelta;
use chrono::Utc;

use margaret_oidc_provider::authenticated_end_user::AuthenticatedEndUser;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::end_user_subject::END_USER_SUBJECT;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn asks_to_authenticate_again_after_the_maximum_authentication_age() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            authorization_request(&with_parameter(portal_parameters(), "max_age", "60")),
            &EndUserAuthentication::Authenticated(AuthenticatedEndUser {
                authenticated_at: Utc::now() - TimeDelta::hours(1),
                subject: END_USER_SUBJECT,
            }),
        )
        .await
        .expect("the authorization reaches its state");

    assert!(matches!(
        outcome,
        AuthorizationOutcome::AuthenticationRequired { .. }
    ));

    fixture.stop().await;
}
