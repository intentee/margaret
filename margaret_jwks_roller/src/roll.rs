use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_keygen::signing_curve::SigningCurve;

use crate::jwks_secret_storage::JwksSecretStorage;
use crate::loaded_secret::LoadedSecret;
use crate::roller_error::RollerError;

fn load_or_fresh(
    storage: &dyn JwksSecretStorage,
    curve: SigningCurve,
    rsa_keys: &dyn ProvidesRsaSigningKeys,
) -> Result<JwksSecret, RollerError> {
    match storage
        .load(rsa_keys)
        .map_err(|source| RollerError::SecretLoad {
            source: source.into(),
        })? {
        LoadedSecret::Present(loaded) => Ok(*loaded),
        LoadedSecret::Absent => {
            JwksSecret::fresh(curve, rsa_keys).map_err(RollerError::KeyGeneration)
        }
    }
}

fn next_secret(
    storage: &dyn JwksSecretStorage,
    holder: &JwksSecretHolder,
    curve: SigningCurve,
    rsa_keys: &dyn ProvidesRsaSigningKeys,
) -> Result<JwksSecret, RollerError> {
    match holder.get() {
        Some(current) => current.rotate(rsa_keys).map_err(RollerError::KeyGeneration),
        None => load_or_fresh(storage, curve, rsa_keys),
    }
}

/// # Errors
///
/// Returns `RollerError::SecretPersist`.
pub fn roll(
    storage: &dyn JwksSecretStorage,
    holder: &JwksSecretHolder,
    curve: SigningCurve,
    rsa_keys: &dyn ProvidesRsaSigningKeys,
) -> Result<Arc<JwksSecret>, RollerError> {
    let next = Arc::new(next_secret(storage, holder, curve, rsa_keys)?);

    storage
        .persist(&next)
        .map_err(|source| RollerError::SecretPersist {
            source: source.into(),
        })?;
    holder.set(Some(next.clone()));

    Ok(next)
}
