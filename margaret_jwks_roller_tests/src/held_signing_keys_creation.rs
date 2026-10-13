use margaret::framework::active_record::model::Model;
use margaret_database::transaction::Transaction;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_key_set_name::SIGNING_KEY_SET_NAME;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;

/// # Panics
///
/// Panics when the placeholder keys cannot be written.
pub async fn held_signing_keys_creation(transaction: &Transaction<'_>, placeholder: &JwksSecret) {
    SigningKeySet {
        name: SIGNING_KEY_SET_NAME.to_string(),
        generation: 1,
        document: SigningKeysRevision::from_secret(placeholder)
            .expect("the placeholder serializes")
            .document,
    }
    .insert()
    .run(transaction)
    .await
    .expect("the placeholder keys are written");
}
