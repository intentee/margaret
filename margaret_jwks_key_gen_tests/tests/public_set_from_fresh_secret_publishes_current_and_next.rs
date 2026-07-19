use anyhow::Result;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;

#[test]
fn public_set_from_fresh_secret_publishes_current_and_next() -> Result<()> {
    let secret = JwksSecret::fresh(Curve::P256)?;
    let current_kid = secret.current.signing.kid.clone();
    let next_kid = secret.next.signing.kid.clone();

    assert_eq!(secret.previous.signing.kid, current_kid);

    let set = JwkPublicSet::from(secret);

    assert_eq!(set.keys.len(), 2);
    assert!(set.find_by_kid(&current_kid).is_some());
    assert!(set.find_by_kid(&next_kid).is_some());

    Ok(())
}
