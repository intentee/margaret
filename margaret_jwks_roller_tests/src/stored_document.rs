use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::secret_text::SecretText;
use margaret_database::database::Database;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_key_set_name::SIGNING_KEY_SET_NAME;

/// # Panics
///
/// Panics when the document cannot be stored.
pub async fn stored_document(database: &Database, document: SecretText) {
    SigningKeySet {
        name: SIGNING_KEY_SET_NAME.to_string(),
        generation: 1,
        document,
    }
    .insert()
    .run(database)
    .await
    .expect("the document is stored");
}
