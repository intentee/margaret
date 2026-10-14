use margaret_database::database::Database;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys_tests::sealed_document_revision::sealed_document_revision;

/// # Panics
///
/// Panics when the document cannot be stored.
pub async fn stored_document(database: &Database, document: &[u8]) {
    SigningKeySet::create(
        database,
        &sealed_document_revision(SigningKeysGeneration::FIRST, document),
    )
    .await
    .expect("the document is stored");
}
