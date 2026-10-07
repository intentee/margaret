use cookie::SameSite;
use cookie::time::Duration;

use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in_tests::begin_sign_in::begin_sign_in;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;

#[tokio::test]
async fn redirects_to_the_authorization_endpoint() {
    let fixture = SignInFixture::start(secret_basic_authentication()).await;
    let SignInBeginning::Redirected(response) = begin_sign_in(&fixture.flow).await else {
        panic!("the sign-in redirects to the authorization endpoint");
    };
    let begun = BegunSignIn::of(&response);

    fixture.server.stop().await;

    assert_eq!(response.status(), 303);
    assert_eq!(begun.authorization_parameter("client_id"), "client:id");
    assert_eq!(
        begun.authorization_parameter("code_challenge_method"),
        "S256"
    );
    assert_eq!(
        begun.authorization_parameter("redirect_uri"),
        "https://client.example/callback"
    );
    assert_eq!(begun.authorization_parameter("response_type"), "code");
    assert_eq!(begun.authorization_parameter("scope"), "openid profile");
    assert_ne!(
        begun.authorization_parameter("nonce"),
        begun.authorization_parameter("state")
    );
    assert!(begun.cookie.name().starts_with("__Host-margaret-sign-in-"));
    assert_eq!(begun.cookie.domain(), None);
    assert_eq!(begun.cookie.http_only(), Some(true));
    assert_eq!(begun.cookie.max_age(), Some(Duration::minutes(10)));
    assert_eq!(begun.cookie.path(), Some("/"));
    assert_eq!(begun.cookie.same_site(), Some(SameSite::Lax));
    assert_eq!(begun.cookie.secure(), Some(true));
}
