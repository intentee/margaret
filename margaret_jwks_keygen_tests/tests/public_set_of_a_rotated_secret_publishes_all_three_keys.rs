use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;

#[test]
fn public_set_of_a_rotated_secret_publishes_all_three_keys() -> Result<()> {
    let fresh = JwksSecret::fresh(Curve::P256)?;
    let rotated = fresh.rotate()?;

    assert_eq!(
        rotated.public_jwks().keys(),
        [
            rotated.current().public_jwk().clone(),
            fresh.current().public_jwk().clone(),
            rotated.next().public_jwk().clone()
        ]
    );

    Ok(())
}
