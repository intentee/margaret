use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;

use crate::fixture_signing_keys_encryption_key::fixture_signing_keys_encryption_key;

/// # Panics
///
/// Panics when the secret cannot be sealed.
#[must_use]
pub fn sealed_revision(secret: &JwksSecret) -> SigningKeysRevision {
    SigningKeysRevision::sealed(secret, &fixture_signing_keys_encryption_key())
        .expect("the secret is sealed")
}
