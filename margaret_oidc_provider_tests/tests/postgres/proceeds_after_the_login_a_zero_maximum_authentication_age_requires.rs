use chrono::TimeDelta;
use chrono::Utc;
use serde_json::Map;
use serde_json::Value;
use url::Url;

use margaret_oidc_provider::authenticated_end_user::AuthenticatedEndUser;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider_tests::end_user_subject::END_USER_SUBJECT;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::validated_form::validated_form;
use margaret_oidc_provider_tests::with_parameter::with_parameter;

fn end_user_authenticated(ago: TimeDelta) -> EndUserAuthentication {
    EndUserAuthentication::Authenticated(AuthenticatedEndUser {
        authenticated_at: Utc::now() - ago,
        subject: END_USER_SUBJECT,
    })
}

#[tokio::test]
async fn proceeds_after_the_login_a_zero_maximum_authentication_age_requires() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::AuthenticationRequired { return_to } = fixture
        .authorization
        .authorize(
            validated_form(&with_parameter(portal_parameters(), "max_age", "0")),
            &end_user_authenticated(TimeDelta::hours(1)),
        )
        .await
        .expect("the authorization reaches its state")
    else {
        panic!("a zero maximum authentication age requires a login");
    };
    let continued: Map<String, Value> = Url::parse(&return_to)
        .expect("the continuation is a url")
        .query_pairs()
        .map(|(name, value)| (name.into_owned(), Value::String(value.into_owned())))
        .collect();

    assert!(matches!(
        fixture
            .authorization
            .authorize(
                validated_form(&Value::Object(continued)),
                &end_user_authenticated(TimeDelta::milliseconds(1)),
            )
            .await
            .expect("the continued authorization reaches its state"),
        AuthorizationOutcome::Redirected(_)
    ));

    fixture.stop().await;
}
