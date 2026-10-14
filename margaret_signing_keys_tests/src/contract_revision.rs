use std::iter;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;

use crate::sealed_revision::sealed_revision;

/// # Panics
///
/// Panics when the fixture secret cannot be rolled.
#[must_use]
pub fn contract_revision(rolls: usize) -> SigningKeysRevision {
    sealed_revision(
        &iter::successors(Some(fresh_secret(SigningCurve::P256)), |secret| {
            Some(rolled_secret(secret))
        })
        .nth(rolls)
        .expect("the fixture secret rolls"),
    )
}
