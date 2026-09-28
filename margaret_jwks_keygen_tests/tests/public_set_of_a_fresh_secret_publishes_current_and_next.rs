use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;

#[test]
fn public_set_of_a_fresh_secret_publishes_current_and_next() -> Result<()> {
    let secret = JwksSecret::fresh(Curve::P256)?;

    assert_eq!(
        secret.public_jwks().keys(),
        [
            secret.current().public_jwk().clone(),
            secret.next().public_jwk().clone()
        ]
    );

    Ok(())
}
