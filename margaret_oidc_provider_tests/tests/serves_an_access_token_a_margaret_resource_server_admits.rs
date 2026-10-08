use serde::Deserialize;

use margaret_bearer_token_verification::bearer_token_routing::BearerTokenRouting;
use margaret_bearer_token_verification::route_bearer_token::route_bearer_token;
use margaret_http::token_admission::TokenAdmission;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider_tests::authentication_claims::AuthenticationClaims;
use margaret_oidc_provider_tests::end_user_subject::END_USER_SUBJECT;
use margaret_oidc_provider_tests::margaret_client::MargaretClient;
use margaret_oidc_provider_tests::portal_callback::PORTAL_CALLBACK;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::request_authorized_by::request_authorized_by;
use margaret_oidc_provider_tests::sign_in_callback::sign_in_callback;
use margaret_oidc_provider_tests::signed_in_end_user::signed_in_end_user;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_validation::validate::validate;

#[derive(Deserialize)]
struct ResourceClaims {
    client_id: String,
    sub: String,
}

#[tokio::test]
async fn serves_an_access_token_a_margaret_resource_server_admits() {
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
    let trusted_issuer = client.server.trusted_issuer.as_ref();
    let request = request_authorized_by(&format!("Bearer {}", signed_in.access_token.secret()));
    let BearerTokenRouting::Routed(routed) =
        route_bearer_token(request.inputs.server.authorization(), &[trusted_issuer])
            .expect("the system clock reads as a numeric date")
    else {
        panic!("the access token is routed to the provider");
    };
    let TokenAdmission::Admitted(verified) = routed
        .admit::<ResourceClaims, AccessTokenProfile>(trusted_issuer)
        .await
    else {
        panic!("the resource server admits the access token");
    };

    assert_eq!(verified.claims.client_id, "portal");
    assert_eq!(verified.claims.sub, END_USER_SUBJECT.to_string());

    client.stop().await;
    fixture.stop().await;
}
