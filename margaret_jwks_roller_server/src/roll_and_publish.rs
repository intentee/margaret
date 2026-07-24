use bytes::Bytes;

use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::roll::roll;

use crate::jwks_curve::JWKS_CURVE;
use crate::jwks_document_holder::JwksDocumentHolder;
use crate::jwks_roller_server_error::JwksRollerServerError;

pub fn roll_and_publish(
    storage: &dyn JwksSecretStorage,
    jwks_secret_holder: &JwksSecretHolder,
    jwks_document_holder: &JwksDocumentHolder,
) -> Result<(), JwksRollerServerError> {
    let rolled =
        roll(storage, jwks_secret_holder, JWKS_CURVE).map_err(JwksRollerServerError::SecretRoll)?;

    serde_json::to_vec(&PublicJwks::from(rolled))
        .map_err(JwksRollerServerError::DocumentSerialization)
        .map(|document| jwks_document_holder.set(Some(Bytes::from(document))))
}
