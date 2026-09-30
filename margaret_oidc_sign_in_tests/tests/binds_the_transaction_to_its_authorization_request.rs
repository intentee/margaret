use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use oauth2::PkceCodeChallenge;
use oauth2::PkceCodeVerifier;
use serde_json::Value;

use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in_tests::begin_sign_in::begin_sign_in;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;

#[tokio::test]
async fn binds_the_transaction_to_its_authorization_request() {
    let fixture = SignInFixture::start(secret_basic_client()).await;
    let SignInBeginning::Redirected(response) = begin_sign_in(&fixture.flow).await else {
        panic!("the sign-in redirects to the authorization endpoint");
    };
    let begun = BegunSignIn::of(&response);

    fixture.server.stop().await;

    let payload = begun
        .cookie
        .value()
        .split('.')
        .nth(1)
        .expect("the transaction is a compact jws");
    let transaction: Value = serde_json::from_slice(
        &Base64UrlUnpadded::decode_vec(payload).expect("the payload is base64url"),
    )
    .expect("the transaction claims are json");
    let verifier = transaction["pkce_verifier"]
        .as_str()
        .expect("the transaction holds the pkce verifier");

    assert_eq!(
        PkceCodeChallenge::from_code_verifier_sha256(&PkceCodeVerifier::new(verifier.to_string()))
            .as_str(),
        begun.authorization_parameter("code_challenge")
    );
    assert_eq!(transaction["nonce"], begun.authorization_parameter("nonce"));
    assert_eq!(transaction["state"], begun.authorization_parameter("state"));
    assert_eq!(transaction["callback"], "https://client.example/callback");
}
