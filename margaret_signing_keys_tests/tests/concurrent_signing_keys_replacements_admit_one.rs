use futures_util::future::join_all;

use margaret_database_tests::racing_instances::RACING_INSTANCES;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_creation::SigningKeysCreation;
use margaret_signing_keys::signing_keys_replacement::SigningKeysReplacement;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_tests::contract_revision::contract_revision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

#[tokio::test]
async fn concurrent_signing_keys_replacements_admit_one() {
    let started = started_with_signing_keys().await;
    let database = started.database.as_ref();

    assert_eq!(
        SigningKeySet::create(database, &contract_revision(0))
            .await
            .expect("the signing keys are created"),
        SigningKeysCreation::Created
    );

    let revisions: Vec<SigningKeysRevision> = (0..RACING_INSTANCES)
        .map(|_| contract_revision(1))
        .collect();
    let replaced: Vec<&SigningKeysRevision> =
        join_all(revisions.iter().map(|revision| async move {
            (SigningKeySet::replace(database, SigningKeysGeneration::FIRST, revision)
                .await
                .expect("the signing keys replacement completes")
                == SigningKeysReplacement::Replaced)
                .then_some(revision)
        }))
        .await
        .into_iter()
        .flatten()
        .collect();

    assert_eq!(replaced.len(), 1);
    assert_eq!(
        StoredRevision::loaded(database).await,
        StoredRevision::of(replaced[0])
    );
}
