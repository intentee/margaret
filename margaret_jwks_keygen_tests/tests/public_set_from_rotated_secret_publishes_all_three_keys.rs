use anyhow::Result;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;

#[test]
fn public_set_from_rotated_secret_publishes_all_three_keys() -> Result<()> {
    let rotated = JwksSecret::fresh(Curve::P256)?.rotate()?;
    let current_kid = rotated.current.signing.kid.clone();
    let next_kid = rotated.next.signing.kid.clone();
    let previous_kid = rotated.previous.signing.kid.clone();

    let set = PublicJwks::from(rotated);

    assert_eq!(set.keys().len(), 3);
    assert!(set.find_by_kid(&current_kid).is_some());
    assert!(set.find_by_kid(&next_kid).is_some());
    assert!(set.find_by_kid(&previous_kid).is_some());

    Ok(())
}
