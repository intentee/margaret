use std::io::Write;

use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys::signing_keys_associated_data::signing_keys_associated_data;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_encryption::wiping_buffer::WipingBuffer;

use crate::fixture_signing_keys_encryption_key::fixture_signing_keys_encryption_key;

/// # Panics
///
/// Panics when the document cannot be sealed.
#[must_use]
pub fn sealed_document_revision(
    generation: SigningKeysGeneration,
    document: &[u8],
) -> SigningKeysRevision {
    let mut plaintext = WipingBuffer::default();

    plaintext
        .write_all(document)
        .expect("the document is buffered");

    SigningKeysRevision {
        generation,
        sealed: fixture_signing_keys_encryption_key()
            .seal(plaintext, &signing_keys_associated_data(generation))
            .expect("the document is sealed"),
    }
}
