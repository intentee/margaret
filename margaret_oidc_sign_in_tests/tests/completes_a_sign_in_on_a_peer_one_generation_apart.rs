use std::collections::BTreeMap;
use std::sync::Arc;

use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_secret_store_tests::peer_secrets::PeerSecrets;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::callback_request::callback_request;
use margaret_oidc_sign_in_tests::email_claims::EmailClaims;
use margaret_oidc_sign_in_tests::fixture_sign_in_flow::fixture_sign_in_flow;
use margaret_oidc_sign_in_tests::id_token_claims::id_token_claims;
use margaret_oidc_sign_in_tests::issued_token_answer::issued_token_answer;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;

async fn completed_on_peer(
    beginning: Arc<JwksSecretHolder>,
    completing: Arc<JwksSecretHolder>,
) -> SignInCompletion<EmailClaims> {
    let fixture = SignInFixture::with_secrets(secret_basic_authentication(), beginning).await;
    let SignInBeginning::Redirected(response) = fixture.flow.begin().await else {
        panic!("the sign-in redirects to the authorization endpoint");
    };
    let begun = BegunSignIn::of(&response);

    assert!(
        fixture
            .token_endpoint
            .answer
            .set(issued_token_answer(
                &fixture.issuer_secret.current().sign_json(
                    &id_token_claims(begun.authorization_parameter("nonce")),
                    JwtType::Jwt
                )
            ))
            .is_ok()
    );

    let completion = fixture_sign_in_flow(fixture.client.clone(), completing)
        .complete::<EmailClaims>(&callback_request(
            &begun.cookie_pair(),
            &BTreeMap::from([
                ("code", "SplxlOBeZQQYbYS6WxSbIA"),
                ("iss", "https://localhost"),
                ("state", begun.authorization_parameter("state")),
            ]),
        ))
        .await;

    fixture.server.stop().await;

    completion
}

#[tokio::test]
async fn completes_a_sign_in_begun_on_a_peer_one_generation_behind() {
    let peers = PeerSecrets::one_generation_apart();

    assert!(matches!(
        completed_on_peer(peers.lagging, peers.rolled).await,
        SignInCompletion::SignedIn(_)
    ));
}

#[tokio::test]
async fn completes_a_sign_in_begun_on_a_peer_one_generation_ahead() {
    let peers = PeerSecrets::one_generation_apart();

    assert!(matches!(
        completed_on_peer(peers.rolled, peers.lagging).await,
        SignInCompletion::SignedIn(_)
    ));
}
