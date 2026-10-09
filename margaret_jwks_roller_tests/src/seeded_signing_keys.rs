use margaret_database::database::Database;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;

/// # Panics
///
/// Panics when the secret cannot be serialized or stored.
pub async fn seeded_signing_keys(database: &Database, secret: &JwksSecret) {
    SigningKeySet::create(
        database,
        &SigningKeysRevision::from_secret(secret).expect("the secret serializes"),
    )
    .await
    .expect("the signing keys are stored");
}
