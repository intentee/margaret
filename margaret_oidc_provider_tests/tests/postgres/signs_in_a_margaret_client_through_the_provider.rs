use serde::Deserialize;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider_tests::authentication_claims::AuthenticationClaims;
use margaret_oidc_provider_tests::end_user_subject::END_USER_SUBJECT;
use margaret_oidc_provider_tests::margaret_client::MargaretClient;
use margaret_oidc_provider_tests::portal_callback::PORTAL_CALLBACK;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::sign_in_callback::sign_in_callback;
use margaret_oidc_provider_tests::signed_in_end_user::signed_in_end_user;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret_oidc_sign_in::userinfo_fetch::UserinfoFetch;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_validation::validate::validate;

#[derive(Deserialize)]
struct NameClaims {
    name: String,
}

#[tokio::test]
async fn signs_in_a_margaret_client_through_the_provider() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let client = MargaretClient::of_portal(&fixture).await;
    let flow = client.sign_in_flow(PORTAL_CALLBACK, &["openid", "profile"]);
    let SignInBeginning::Redirected(beginning) = flow.begin().await else {
        panic!("the sign-in redirects to the provider");
    };
    let begun = BegunSignIn::of(&beginning);
    let AuthorizationOutcome::Redirected(callback) = fixture
        .authorization
        .authorize(
            validate(&begun.authorization.parameters),
            &EndUserAuthentication::Authenticated(signed_in_end_user()),
        )
        .await
        .expect("the authorization reaches its state")
    else {
        panic!("the provider redirects back to the portal");
    };
    let SignInCompletion::SignedIn(signed_in) = flow
        .complete::<AuthenticationClaims>(&sign_in_callback(&begun, &callback))
        .await
    else {
        panic!("the portal signs in");
    };
    let UserinfoFetch::Fetched(NameClaims { name }) =
        flow.userinfo::<NameClaims, _>(&signed_in).await
    else {
        panic!("the provider answers userinfo of the signed-in subject");
    };

    assert_eq!(signed_in.subject, END_USER_SUBJECT.to_string());
    assert!(signed_in.claims.auth_time > 0);
    assert_eq!(name, "Ada");

    client.stop().await;
    fixture.stop().await;
}
