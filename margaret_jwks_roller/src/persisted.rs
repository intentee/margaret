use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;

use crate::jwks_secret_storage::JwksSecretStorage;
use crate::roller_error::RollerError;

pub(crate) fn persisted(
    storage: &dyn JwksSecretStorage,
    secret: JwksSecret,
) -> Result<Arc<JwksSecret>, RollerError> {
    storage
        .persist(&secret)
        .map(|()| Arc::new(secret))
        .map_err(|source| RollerError::SecretPersist {
            source: source.into(),
        })
}
