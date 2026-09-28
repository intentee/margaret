use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;

#[test]
fn jwks_secret_rotation_preserves_the_curve() -> Result<()> {
    let rotated = JwksSecret::fresh(Curve::P384)?.rotate()?;

    assert_eq!(rotated.current().signing_key().curve(), Curve::P384);
    assert_eq!(rotated.next().signing_key().curve(), Curve::P384);

    Ok(())
}
