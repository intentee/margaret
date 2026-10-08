use futures_util::future::join_all;

use margaret_jwks_roller::signing_keys_creation::SigningKeysCreation;
use margaret_jwks_roller::signing_keys_revision::SigningKeysRevision;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

use crate::contract_revision::contract_revision;
use crate::racing_instances::RACING_INSTANCES;
use crate::stored_revision::StoredRevision;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn concurrent_signing_keys_creations_admit_one(store: &dyn StoresSigningKeys) {
    let revisions: Vec<SigningKeysRevision> = (0..RACING_INSTANCES)
        .map(|_| contract_revision(0))
        .collect();
    let created: Vec<&SigningKeysRevision> =
        join_all(revisions.iter().map(|revision| async move {
            (store
                .create_signing_keys(revision)
                .await
                .expect("the store creates signing keys")
                == SigningKeysCreation::Created)
                .then_some(revision)
        }))
        .await
        .into_iter()
        .flatten()
        .collect();

    assert_eq!(created.len(), 1);
    assert_eq!(
        StoredRevision::loaded(store).await,
        StoredRevision::of(created[0])
    );
}
