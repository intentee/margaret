use futures_util::future::join_all;

use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::signing_keys_creation::SigningKeysCreation;
use margaret_jwks_roller::signing_keys_replacement::SigningKeysReplacement;
use margaret_jwks_roller::signing_keys_revision::SigningKeysRevision;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

use crate::contract_revision::contract_revision;
use crate::racing_instances::RACING_INSTANCES;
use crate::stored_revision::StoredRevision;

/// # Panics
///
/// Panics when the store breaks this clause of its contract or cannot be reached.
pub async fn concurrent_signing_keys_replacements_admit_one(store: &dyn StoresSigningKeys) {
    assert_eq!(
        store
            .create_signing_keys(&contract_revision(0))
            .await
            .expect("the store creates signing keys"),
        SigningKeysCreation::Created
    );

    let revisions: Vec<SigningKeysRevision> = (0..RACING_INSTANCES)
        .map(|_| contract_revision(1))
        .collect();
    let replaced: Vec<&SigningKeysRevision> =
        join_all(revisions.iter().map(|revision| async move {
            (store
                .replace_signing_keys(SigningKeysGeneration::FIRST, revision)
                .await
                .expect("the store replaces signing keys")
                == SigningKeysReplacement::Replaced)
                .then_some(revision)
        }))
        .await
        .into_iter()
        .flatten()
        .collect();

    assert_eq!(replaced.len(), 1);
    assert_eq!(
        StoredRevision::loaded(store).await,
        StoredRevision::of(replaced[0])
    );
}
