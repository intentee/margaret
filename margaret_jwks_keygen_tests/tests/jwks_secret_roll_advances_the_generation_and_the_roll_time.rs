use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;

#[test]
fn jwks_secret_roll_advances_the_generation_and_the_roll_time() {
    let secret = fresh_secret(SigningCurve::P256);
    let rolled = rolled_secret(&secret);

    assert_eq!(rolled.generation(), SigningKeysGeneration::new(2));
    assert_eq!(
        rolled.rolled_at(),
        secret.rolled_at().after(JWKS_ROLL_INTERVAL)
    );
    assert_eq!(rolled.current().kid(), secret.next().kid());
    assert_eq!(rolled.retired()[0].retired_at(), rolled.rolled_at());
}
