use chrono::Utc;
use serde_json::Value;

use margaret::framework::jwks_roller::constants::JWKS_ROLL_INTERVAL;
use margaret::framework::registered_claims::numeric_date::NumericDate;
use margaret::framework::signing_keys::signing_key_set::SigningKeySet;
use margaret::framework::signing_keys::signing_keys_replacement::SigningKeysReplacement;
use margaret_signing_keys_tests::fixture_signing_keys_encryption_key::fixture_signing_keys_encryption_key;
use margaret_signing_keys_tests::sealed_document_revision::sealed_document_revision;

use crate::cluster::Cluster;
use crate::stored_signing_keys_revision::stored_signing_keys_revision;

/// # Panics
///
/// Panics when the signing keys cannot be made overdue in the cluster database.
pub async fn overdue_signing_keys(cluster: &Cluster) {
    let database = cluster.database.database.as_ref();
    let revision = stored_signing_keys_revision(
        SigningKeySet::load(database)
            .await
            .expect("the signing keys load"),
    );
    let generation = revision.generation;
    let mut document = serde_json::to_value(
        revision
            .opened(&fixture_signing_keys_encryption_key())
            .expect("the signing keys open under the cluster key"),
    )
    .expect("the opened signing keys serialize");

    document["rolled_at"] =
        Value::from(NumericDate::from(Utc::now() - JWKS_ROLL_INTERVAL).seconds_since_epoch() - 1);

    assert_eq!(
        SigningKeySet::replace(
            database,
            generation,
            &sealed_document_revision(generation, document.to_string().as_bytes()),
        )
        .await
        .expect("the signing keys become overdue"),
        SigningKeysReplacement::Replaced
    );
}
