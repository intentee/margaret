use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;

#[test]
fn jwks_secret_roll_preserves_the_curve() {
    let rolled = rolled_secret(&fresh_secret(SigningCurve::P384));

    assert_eq!(rolled.current().signing_key().curve(), SigningCurve::P384);
    assert_eq!(rolled.next().signing_key().curve(), SigningCurve::P384);
}
