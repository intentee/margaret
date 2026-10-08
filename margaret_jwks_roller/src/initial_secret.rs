use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_keygen::signing_curve::SigningCurve;

use crate::persisted::persisted;
use crate::restored_secret::restored_secret;
use crate::roller_error::RollerError;
use crate::stored_signing_keys::StoredSigningKeys;
use crate::stores_signing_keys::StoresSigningKeys;

/// # Errors
///
/// Returns `RollerError::SecretLoad` when the stored keys cannot be loaded,
/// `RollerError::DocumentMalformed`, `RollerError::DocumentRestore` or
/// `RollerError::CurveMismatch` when they cannot be restored, `RollerError::KeyGeneration` when
/// a fresh secret cannot be generated, and `RollerError::SecretPersist` when a fresh secret cannot
/// be stored.
pub async fn initial_secret(
    storage: &dyn StoresSigningKeys,
    curve: SigningCurve,
    rsa_keys: &dyn ProvidesRsaSigningKeys,
) -> Result<Arc<JwksSecret>, RollerError> {
    match storage
        .load_signing_keys()
        .await
        .map_err(|source| RollerError::SecretLoad { source })?
    {
        StoredSigningKeys::Stored(document) => {
            restored_secret(&document, curve, rsa_keys).map(Arc::new)
        }
        StoredSigningKeys::Absent => {
            let fresh = JwksSecret::fresh(curve, rsa_keys).map_err(RollerError::KeyGeneration)?;

            persisted(storage, fresh).await
        }
    }
}
