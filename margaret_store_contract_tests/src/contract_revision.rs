use std::iter;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_roller::signing_keys_revision::SigningKeysRevision;

/// # Panics
///
/// Panics when the fixture secret cannot be serialized.
#[must_use]
pub fn contract_revision(rolls: usize) -> SigningKeysRevision {
    SigningKeysRevision::from_secret(
        &iter::successors(Some(fresh_secret(SigningCurve::P256)), |secret| {
            Some(rolled_secret(secret))
        })
        .nth(rolls)
        .expect("the fixture secret rolls"),
    )
    .expect("the fixture secret serializes")
}
