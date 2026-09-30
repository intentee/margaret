use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::loaded_secret::LoadedSecret;

use crate::storage_backend_error::StorageBackendError;

pub struct UnreachableJwksSecretStorage;

impl JwksSecretStorage for UnreachableJwksSecretStorage {
    fn load(&self, _rsa_keys: &dyn ProvidesRsaSigningKeys) -> anyhow::Result<LoadedSecret> {
        Err(StorageBackendError::Unreachable.into())
    }

    fn persist(&self, _secret: &JwksSecret) -> anyhow::Result<()> {
        Err(StorageBackendError::Unreachable.into())
    }
}
