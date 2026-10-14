use std::sync::Arc;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_signing_keys_tests::fixture_signing_keys_encryption_key::fixture_signing_keys_encryption_key;
use margaret_signing_keys_tests::sealed_revision::sealed_revision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

#[tokio::test]
async fn jwks_roller_restores_the_stored_keys_across_a_restart() {
    let started = started_with_signing_keys().await;
    let first_start = JwksRoller::create(
        Arc::clone(&started.database),
        Arc::new(FixtureRsaSigningKeys::default()),
        fixture_signing_keys_encryption_key(),
    )
    .await
    .expect("the first start creates the keys")
    .jwks_secret_holder()
    .get();
    let restart = JwksRoller::create(
        started.separate_pool().await,
        Arc::new(FixtureRsaSigningKeys::default()),
        fixture_signing_keys_encryption_key(),
    )
    .await
    .expect("the restart restores the keys")
    .jwks_secret_holder()
    .get();

    assert_eq!(restart.current().kid(), first_start.current().kid());
    assert_eq!(restart.next().kid(), first_start.next().kid());
    assert_eq!(
        StoredRevision::loaded(&started.database).await,
        StoredRevision::of(&sealed_revision(&first_start))
    );
}
