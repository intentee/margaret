use margaret_http::token_admission::TokenAdmission;
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
use margaret_token_introspection::introspect_bearer_token::introspect_bearer_token;
use margaret_validation::validate::validate;

#[tokio::test]
async fn introspects_an_access_token_for_a_margaret_resource_server() {
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
    let request = request_authorized_by(&format!("Bearer {}", signed_in.access_token.secret()));
    let TokenAdmission::Admitted(introspected) = introspect_bearer_token::<serde_json::Value>(
        request.inputs.server.authorization(),
        &client.server,
    )
    .await
    else {
        panic!("the provider reports the access token as active");
    };

    assert_eq!(
        introspected.subject.as_deref(),
        Some(END_USER_SUBJECT.to_string().as_str())
    );

    client.stop().await;
    fixture.stop().await;
}
