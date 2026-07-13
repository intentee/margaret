use anyhow::Result;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;

#[test]
fn public_set_from_fresh_secret_has_single_key() -> Result<()> {
    let secret = JwksSecret::fresh(Curve::P256)?;
    let current_kid = secret.current.signing.kid.clone();

    let set = JwkPublicSet::from(secret);

    assert_eq!(set.keys.len(), 1);
    assert!(set.find_by_kid(&current_kid).is_some());

    Ok(())
}
