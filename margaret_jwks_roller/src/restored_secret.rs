use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_keygen::signing_curve::SigningCurve;

use crate::roller_error::RollerError;
use crate::signing_keys_document::SigningKeysDocument;

pub(crate) fn restored_secret(
    document: &SigningKeysDocument,
    curve: SigningCurve,
    rsa_keys: &dyn ProvidesRsaSigningKeys,
) -> Result<JwksSecret, RollerError> {
    let secret = serde_json::from_str::<PersistedJwksSecret>(document.json())
        .map_err(|source| RollerError::DocumentMalformed { source })?
        .into_secret(rsa_keys)
        .map_err(|source| RollerError::DocumentRestore { source })?;
    let stored = secret.current().signing_key().curve();

    if stored == curve {
        Ok(secret)
    } else {
        Err(RollerError::CurveMismatch {
            pinned: curve,
            stored,
        })
    }
}
