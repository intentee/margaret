use std::future::ready;
use std::sync::Arc;

use futures_util::TryFutureExt as _;
use zeroize::Zeroizing;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;

use crate::roller_error::RollerError;
use crate::signing_keys_document::SigningKeysDocument;
use crate::stores_signing_keys::StoresSigningKeys;

pub(crate) async fn persisted(
    storage: &dyn StoresSigningKeys,
    secret: JwksSecret,
) -> Result<Arc<JwksSecret>, RollerError> {
    ready(
        serde_json::to_string(&PersistedJwksSecret::from_secret(&secret))
            .map(|json| SigningKeysDocument::new(Zeroizing::new(json)))
            .map_err(RollerError::DocumentSerialization),
    )
    .and_then(|document| async move {
        storage
            .store_signing_keys(&document)
            .await
            .map_err(|source| RollerError::SecretPersist { source })
    })
    .await
    .map(|()| Arc::new(secret))
}
