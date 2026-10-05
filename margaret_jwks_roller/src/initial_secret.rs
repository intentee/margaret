use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_keygen::signing_curve::SigningCurve;

use crate::jwks_secret_storage::JwksSecretStorage;
use crate::loaded_secret::LoadedSecret;
use crate::persisted::persisted;
use crate::roller_error::RollerError;

fn loaded_or_fresh(
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

/// # Errors
///
/// Returns `RollerError::SecretLoad` when the persisted secret cannot be read,
/// `RollerError::KeyGeneration` when a fresh secret cannot be generated, and
/// `RollerError::SecretPersist` when the secret cannot be persisted.
pub fn initial_secret(
    storage: &dyn JwksSecretStorage,
    curve: SigningCurve,
    rsa_keys: &dyn ProvidesRsaSigningKeys,
) -> Result<Arc<JwksSecret>, RollerError> {
    persisted(storage, loaded_or_fresh(storage, curve, rsa_keys)?)
}
