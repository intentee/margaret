use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;

use crate::jwks_secret_storage::JwksSecretStorage;
use crate::persisted::persisted;
use crate::roller_error::RollerError;

/// # Errors
///
/// Returns `RollerError::KeyGeneration` when the next key cannot be generated, and
/// `RollerError::SecretPersist` when the rotated secret cannot be persisted.
pub fn roll(
    storage: &dyn JwksSecretStorage,
    holder: &JwksSecretHolder,
    rsa_keys: &dyn ProvidesRsaSigningKeys,
) -> Result<Arc<JwksSecret>, RollerError> {
    let rotated = holder
        .get()
        .rotate(rsa_keys)
        .map_err(RollerError::KeyGeneration)
        .and_then(|next| persisted(storage, next))?;

    holder.set(rotated.clone());

    Ok(rotated)
}
