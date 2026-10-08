use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;

use crate::roller_error::RollerError;
use crate::signing_key_retention::signing_key_retention;
use crate::signing_keys_revision::SigningKeysRevision;

pub(crate) fn restored_secret(
    SigningKeysRevision {
        document,
        generation,
    }: &SigningKeysRevision,
    curve: SigningCurve,
) -> Result<JwksSecret, RollerError> {
    let secret = serde_json::from_str::<PersistedJwksSecret>(document.json())
        .map_err(|source| RollerError::DocumentMalformed { source })?
        .into_secret(*generation, signing_key_retention())
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
