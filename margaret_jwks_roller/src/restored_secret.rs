use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_encryption::signing_keys_encryption_key::SigningKeysEncryptionKey;

use crate::roller_error::RollerError;
use crate::signing_key_retention::signing_key_retention;

pub(crate) fn restored_secret(
    revision: SigningKeysRevision,
    encryption_key: &SigningKeysEncryptionKey,
    curve: SigningCurve,
) -> Result<JwksSecret, RollerError> {
    let generation = revision.generation;
    let secret = revision
        .opened(encryption_key)
        .map_err(|source| RollerError::DocumentOpen { source })?
        .into_secret(generation, signing_key_retention())
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
