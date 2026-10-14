use std::fmt::Debug;
use std::fmt::Formatter;
use std::mem;
use std::str::FromStr;

use aws_lc_rs::aead::AES_256_GCM;
use aws_lc_rs::aead::Aad;
use aws_lc_rs::aead::Nonce;
use aws_lc_rs::aead::RandomizedNonceKey;
use base64ct::Base64;
use base64ct::Encoding;
use zeroize::Zeroizing;

use crate::sealed_document::SealedDocument;
use crate::signing_keys_encryption_error::SigningKeysEncryptionError;
use crate::wiping_buffer::WipingBuffer;

pub struct SigningKeysEncryptionKey {
    key: RandomizedNonceKey,
}

impl SigningKeysEncryptionKey {
    /// # Errors
    ///
    /// Returns `SigningKeysEncryptionError::Open` when the document was sealed under another key
    /// or another associated data, or was altered after sealing.
    pub fn open(
        &self,
        SealedDocument { ciphertext, nonce }: SealedDocument,
        associated_data: &[u8],
    ) -> Result<Zeroizing<Vec<u8>>, SigningKeysEncryptionError> {
        let mut document = Zeroizing::new(ciphertext);
        let plaintext_length = self
            .key
            .open_in_place(
                Nonce::from(&nonce),
                Aad::from(associated_data),
                &mut document,
            )
            .map_err(SigningKeysEncryptionError::Open)?
            .len();

        document.truncate(plaintext_length);

        Ok(document)
    }

    /// # Errors
    ///
    /// Returns `SigningKeysEncryptionError::Seal` when AES-256-GCM cannot seal the document.
    pub fn seal(
        &self,
        mut plaintext: WipingBuffer,
        associated_data: &[u8],
    ) -> Result<SealedDocument, SigningKeysEncryptionError> {
        plaintext.reserve(AES_256_GCM.tag_len());

        let mut document = plaintext.into_bytes();

        self.key
            .seal_in_place_append_tag(Aad::from(associated_data), &mut *document)
            .map(|nonce| SealedDocument {
                ciphertext: mem::take(&mut *document),
                nonce: *nonce.as_ref(),
            })
            .map_err(SigningKeysEncryptionError::Seal)
    }
}

impl Debug for SigningKeysEncryptionKey {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SigningKeysEncryptionKey")
            .finish_non_exhaustive()
    }
}

impl FromStr for SigningKeysEncryptionKey {
    type Err = SigningKeysEncryptionError;

    fn from_str(encoded: &str) -> Result<Self, Self::Err> {
        let material = Base64::decode_vec(encoded)
            .map(Zeroizing::new)
            .map_err(|source| SigningKeysEncryptionError::KeyNotBase64 { source })?;

        if material.len() != AES_256_GCM.key_len() {
            return Err(SigningKeysEncryptionError::KeyLength {
                expected: AES_256_GCM.key_len(),
                length: material.len(),
            });
        }

        RandomizedNonceKey::new(&AES_256_GCM, &material)
            .map(|key| Self { key })
            .map_err(SigningKeysEncryptionError::KeyRejected)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use aws_lc_rs::error::Unspecified;

    use super::SigningKeysEncryptionKey;
    use crate::sealed_document::SealedDocument;
    use crate::signing_keys_encryption_error::SigningKeysEncryptionError;
    use crate::wiping_buffer::WipingBuffer;

    const KEY: &str = "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=";
    const OTHER_KEY: &str = "Hx4dHBsaGRgXFhUUExIREA8ODQwLCgkIBwYFBAMCAQA=";
    const DOCUMENT: &[u8] = br#"{"keys":"private"}"#;

    fn key(encoded: &str) -> SigningKeysEncryptionKey {
        encoded.parse().expect("the fixture key parses")
    }

    fn sealed(key: &SigningKeysEncryptionKey, associated_data: &[u8]) -> SealedDocument {
        let mut plaintext = WipingBuffer::default();

        plaintext
            .write_all(DOCUMENT)
            .expect("the plaintext is buffered");

        key.seal(plaintext, associated_data)
            .expect("the document is sealed")
    }

    #[test]
    fn opens_the_document_it_sealed() {
        let key = key(KEY);

        assert_eq!(
            *key.open(sealed(&key, b"jwks:7"), b"jwks:7")
                .expect("the document opens"),
            DOCUMENT
        );
    }

    #[test]
    fn hides_the_document_in_what_it_stores() {
        let SealedDocument { ciphertext, .. } = sealed(&key(KEY), b"jwks:7");

        assert!(
            !ciphertext
                .windows(DOCUMENT.len())
                .any(|window| window == DOCUMENT)
        );
    }

    #[test]
    fn seals_the_same_document_under_a_fresh_nonce_every_time() {
        let key = key(KEY);

        assert_ne!(sealed(&key, b"jwks:7").nonce, sealed(&key, b"jwks:7").nonce);
    }

    #[test]
    fn refuses_a_document_sealed_for_another_generation() {
        let key = key(KEY);

        assert_eq!(
            key.open(sealed(&key, b"jwks:7"), b"jwks:8")
                .expect_err("a document of another generation is refused"),
            SigningKeysEncryptionError::Open(Unspecified)
        );
    }

    #[test]
    fn refuses_a_document_altered_after_sealing() {
        let key = key(KEY);
        let mut altered = sealed(&key, b"jwks:7");

        altered.ciphertext[0] ^= 1;

        assert_eq!(
            key.open(altered, b"jwks:7")
                .expect_err("an altered document is refused"),
            SigningKeysEncryptionError::Open(Unspecified)
        );
    }

    #[test]
    fn refuses_a_document_sealed_under_another_key() {
        assert_eq!(
            key(KEY)
                .open(sealed(&key(OTHER_KEY), b"jwks:7"), b"jwks:7")
                .expect_err("a document of another key is refused"),
            SigningKeysEncryptionError::Open(Unspecified)
        );
    }

    #[test]
    fn rejects_a_key_that_is_not_base64() {
        assert_eq!(
            "not base64!"
                .parse::<SigningKeysEncryptionKey>()
                .expect_err("a key that is not base64 is rejected"),
            SigningKeysEncryptionError::KeyNotBase64 {
                source: base64ct::Error::InvalidEncoding,
            }
        );
    }

    #[test]
    fn rejects_a_key_shorter_than_aes_256_takes() {
        assert_eq!(
            "AAECAwQFBgcICQoLDA0ODw=="
                .parse::<SigningKeysEncryptionKey>()
                .expect_err("a 128-bit key is rejected"),
            SigningKeysEncryptionError::KeyLength {
                expected: 32,
                length: 16,
            }
        );
    }

    #[test]
    fn describes_the_key_without_revealing_it() {
        assert_eq!(format!("{:?}", key(KEY)), "SigningKeysEncryptionKey { .. }");
    }
}
