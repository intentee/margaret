use std::sync::Arc;

use margaret_signing_keys_encryption::signing_keys_encryption_key::SigningKeysEncryptionKey;

use crate::constants::FIXTURE_SIGNING_KEYS_ENCRYPTION_KEY;

/// # Panics
///
/// Panics when the fixture key is not an AES-256 key.
#[must_use]
pub fn fixture_signing_keys_encryption_key() -> Arc<SigningKeysEncryptionKey> {
    Arc::new(
        FIXTURE_SIGNING_KEYS_ENCRYPTION_KEY
            .parse()
            .expect("the fixture key is an AES-256 key"),
    )
}
