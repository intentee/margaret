use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use trzcina::Service;
use trzcina::ServiceBundle;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_roller::initial_secret::initial_secret;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::roll::roll;

use crate::jwks_curve::JWKS_CURVE;
use crate::jwks_document_holder::JwksDocumentHolder;
use crate::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;
use crate::jwks_roller_server_error::JwksRollerServerError;
use crate::jwks_roller_service::JwksRollerService;
use crate::public_jwks_handler::PublicJwksHandler;

fn published_document(secret: &JwksSecret) -> Result<Bytes, JwksRollerServerError> {
    serde_json::to_vec(secret.public_jwks())
        .map(Bytes::from)
        .map_err(JwksRollerServerError::DocumentSerialization)
}

pub struct JwksRollerServerBundle {
    jwks_document_holder: JwksDocumentHolder,
    jwks_secret_holder: JwksSecretHolder,
    rsa_keys: Arc<dyn ProvidesRsaSigningKeys>,
    storage: Arc<dyn JwksSecretStorage>,
}

impl JwksRollerServerBundle {
    /// # Errors
    ///
    /// Returns `JwksRollerServerError::SecretRoll` when the first secret cannot be loaded,
    /// generated or persisted, and `JwksRollerServerError::DocumentSerialization` when its public
    /// document cannot be serialized.
    pub fn new(
        JwksRollerServerBundleParams { rsa_keys, storage }: JwksRollerServerBundleParams,
    ) -> Result<Self, JwksRollerServerError> {
        let secret = initial_secret(storage.as_ref(), JWKS_CURVE, rsa_keys.as_ref())
            .map_err(JwksRollerServerError::SecretRoll)?;

        published_document(&secret).map(|document| Self {
            jwks_document_holder: JwksDocumentHolder::new(document),
            jwks_secret_holder: JwksSecretHolder::new(secret),
            rsa_keys,
            storage,
        })
    }

    #[must_use]
    pub fn jwks_document_holder(&self) -> JwksDocumentHolder {
        self.jwks_document_holder.clone()
    }

    #[must_use]
    pub fn jwks_secret_holder(&self) -> JwksSecretHolder {
        self.jwks_secret_holder.clone()
    }

    #[must_use]
    pub fn public_jwks_handler(&self) -> Arc<PublicJwksHandler> {
        Arc::new(PublicJwksHandler::new(self.jwks_document_holder.clone()))
    }

    /// # Errors
    ///
    /// Returns `JwksRollerServerError::SecretRoll` or `JwksRollerServerError::DocumentSerialization`.
    pub fn roll_and_publish(&self) -> Result<(), JwksRollerServerError> {
        roll(
            self.storage.as_ref(),
            &self.jwks_secret_holder,
            self.rsa_keys.as_ref(),
        )
        .map_err(JwksRollerServerError::SecretRoll)
        .and_then(|rolled| published_document(&rolled))
        .map(|document| self.jwks_document_holder.set(document))
    }
}

#[async_trait]
impl ServiceBundle for JwksRollerServerBundle {
    async fn services(self) -> Result<Vec<Box<dyn Service>>> {
        Ok(vec![Box::new(JwksRollerService { bundle: self })])
    }
}
