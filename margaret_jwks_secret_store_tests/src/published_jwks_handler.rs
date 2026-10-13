use std::sync::Arc;

use bytes::Bytes;

use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_roller_server::jwks_document_holder::JwksDocumentHolder;
use margaret_jwks_roller_server::public_jwks_handler::PublicJwksHandler;

/// # Panics
///
/// Panics when the public key set of the held secret cannot be serialized.
#[must_use]
pub fn published_jwks_handler(secrets: &JwksSecretHolder) -> Arc<PublicJwksHandler> {
    Arc::new(PublicJwksHandler::new(JwksDocumentHolder::new(
        Bytes::from(
            serde_json::to_vec(secrets.get().public_jwks()).expect("the public key set serializes"),
        ),
    )))
}
