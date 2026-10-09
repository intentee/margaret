use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;

#[test]
fn public_set_of_a_rolled_secret_publishes_its_retired_keys() {
    let fresh = fresh_secret(SigningCurve::P256);
    let rolled = rolled_secret(&fresh);

    assert_eq!(
        rolled.public_jwks().keys(),
        [
            rolled.current().public_jwk().clone(),
            rolled.next().public_jwk().clone(),
            fresh.current().public_jwk().clone(),
            rolled.rsa().current().public_jwk().clone(),
            rolled.rsa().next().public_jwk().clone(),
            fresh.rsa().current().public_jwk().clone()
        ]
    );
}
