use anyhow::Result;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;

#[test]
fn public_set_from_rotated_secret_has_two_keys() -> Result<()> {
    let rotated = JwksSecret::fresh(Curve::P256)?.rotate(Curve::P256)?;
    let current_kid = rotated.current.signing.kid.clone();
    let previous_kid = rotated.previous.signing.kid.clone();

    let set = JwkPublicSet::from(rotated);

    assert_eq!(set.keys.len(), 2);
    assert!(set.find_by_kid(&current_kid).is_some());
    assert!(set.find_by_kid(&previous_kid).is_some());

    Ok(())
}
