use margaret::framework::active_record::model::Model;
use margaret_database::transaction::Transaction;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_signing_keys::constants::SIGNING_KEY_SET_NAME;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_encryption::sealed_document::SealedDocument;
use margaret_signing_keys_tests::sealed_revision::sealed_revision;

/// # Panics
///
/// Panics when the placeholder keys cannot be written.
pub async fn held_signing_keys_creation(transaction: &Transaction<'_>, placeholder: &JwksSecret) {
    let SigningKeysRevision {
        sealed: SealedDocument { ciphertext, nonce },
        ..
    } = sealed_revision(placeholder);

    SigningKeySet {
        name: SIGNING_KEY_SET_NAME.to_string(),
        generation: 1,
        nonce: nonce.to_vec(),
        sealed_document: ciphertext,
    }
    .insert()
    .run(transaction)
    .await
    .expect("the placeholder keys are written");
}
