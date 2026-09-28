use std::sync::Arc;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client::verification_key_set_holder::VerificationKeySetHolder;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

#[tokio::test]
async fn public_jwks_verifier_rejects_an_expired_token() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let claims = TestClaims {
        exp: 1_699_999_999,
        sub: "subject".to_string(),
    };
    let token = secret
        .current()
        .sign(&claims)
        .await
        .expect("the claims sign");
    let KeySetParsing::Accepted(key_set) =
        VerificationKeySet::from_jwks(secret.public_jwks().keys().to_vec())
    else {
        panic!("the published key set is accepted");
    };
    let holder = VerificationKeySetHolder::default();

    holder.set(Some(Arc::new(key_set)));

    assert!(matches!(
        PublicJwksVerifier::new(holder).verify::<TestClaims>(&token, test_instant(1_700_000_000)),
        Ok(AccessTokenVerification::Expired)
    ));
}
