use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;

use crate::persisted::persisted;
use crate::roller_error::RollerError;
use crate::stores_signing_keys::StoresSigningKeys;

/// # Errors
///
/// Returns `RollerError::KeyGeneration` when the next key cannot be generated, and
/// `RollerError::DocumentSerialization` or `RollerError::SecretPersist` when the rotated secret
/// cannot be stored.
pub async fn roll(
    storage: &dyn StoresSigningKeys,
    holder: &JwksSecretHolder,
    rsa_keys: &dyn ProvidesRsaSigningKeys,
) -> Result<Arc<JwksSecret>, RollerError> {
    let next = holder
        .get()
        .rotate(rsa_keys)
        .map_err(RollerError::KeyGeneration)?;
    let rotated = persisted(storage, next).await?;

    holder.set(rotated.clone());

    Ok(rotated)
}
