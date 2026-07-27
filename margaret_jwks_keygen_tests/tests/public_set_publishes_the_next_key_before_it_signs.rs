use anyhow::Result;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;

#[test]
fn public_set_publishes_the_next_key_before_it_signs() -> Result<()> {
    let rotated = JwksSecret::fresh(Curve::P256)?.rotate()?;
    let next_kid = rotated.next.signing.kid.clone();
    let signing_kid = rotated.current.signing.kid.clone();

    let published_before_signing = PublicJwks::from(rotated.clone());

    assert_ne!(next_kid, signing_kid);
    assert!(
        published_before_signing
            .find_by_kid(&next_kid)
            .expect("published key ids are unique")
            .is_some()
    );

    let promoted = rotated.rotate()?;

    assert_eq!(promoted.current.signing.kid, next_kid);

    Ok(())
}
