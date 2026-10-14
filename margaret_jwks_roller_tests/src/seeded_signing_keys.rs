use margaret_database::database::Database;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys_tests::sealed_revision::sealed_revision;

/// # Panics
///
/// Panics when the secret cannot be stored.
pub async fn seeded_signing_keys(database: &Database, secret: &JwksSecret) {
    SigningKeySet::create(database, &sealed_revision(secret))
        .await
        .expect("the signing keys are stored");
}
