use async_trait::async_trait;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::initial_secret::initial_secret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller::signing_keys_document::SigningKeysDocument;
use margaret_jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;
use margaret_jwks_roller_tests::storage_backend_error::StorageBackendError;

struct ReadOnlySigningKeys;

#[async_trait]
impl StoresSigningKeys for ReadOnlySigningKeys {
    async fn load_signing_keys(&self) -> anyhow::Result<StoredSigningKeys> {
        Ok(StoredSigningKeys::Absent)
    }

    async fn store_signing_keys(&self, _document: &SigningKeysDocument) -> anyhow::Result<()> {
        Err(StorageBackendError::Unreachable.into())
    }
}

#[tokio::test]
async fn initial_secret_reports_a_store_error_when_the_backend_refuses_the_seed() {
    let Err(error) = initial_secret(
        &ReadOnlySigningKeys,
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .await
    else {
        panic!("the seed cannot be stored");
    };

    assert!(matches!(error, RollerError::SecretPersist { .. }));
}
