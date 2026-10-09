use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_error::SigningKeysError;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

use crate::postgres::beyond_the_database_range::beyond_the_database_range;

#[tokio::test]
async fn refuses_to_create_a_generation_beyond_the_database_range() {
    let started = started_with_signing_keys().await;

    assert!(matches!(
        SigningKeySet::create(&started.database, &beyond_the_database_range()).await,
        Err(SigningKeysError::GenerationOutOfRange { generation, .. })
            if generation == SigningKeysGeneration::new(u64::MAX)
    ));
}
