use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::loaded_secret::LoadedSecret;
use margaret_jwks_roller::roller_error::RollerError;

use crate::storage_backend_error::StorageBackendError;

pub struct UnreachableJwksSecretStorage;

impl JwksSecretStorage for UnreachableJwksSecretStorage {
    fn load(&self) -> Result<LoadedSecret, RollerError> {
        Err(RollerError::SecretLoad {
            source: Box::new(StorageBackendError::Unreachable),
        })
    }

    fn persist(&self, _secret: &JwksSecret) -> Result<(), RollerError> {
        Err(RollerError::SecretPersist {
            source: Box::new(StorageBackendError::Unreachable),
        })
    }
}
