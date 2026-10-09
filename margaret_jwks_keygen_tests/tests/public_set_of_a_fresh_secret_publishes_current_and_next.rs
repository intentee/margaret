use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;

#[test]
fn public_set_of_a_fresh_secret_publishes_current_and_next() {
    let secret = fresh_secret(SigningCurve::P256);

    assert_eq!(
        secret.public_jwks().keys(),
        [
            secret.current().public_jwk().clone(),
            secret.next().public_jwk().clone(),
            secret.rsa().current().public_jwk().clone(),
            secret.rsa().next().public_jwk().clone()
        ]
    );
}
