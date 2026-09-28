use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use trzcina::Service;
use trzcina::ServiceBundle;

use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::roll::roll;

use crate::jwks_curve::JWKS_CURVE;
use crate::jwks_document_holder::JwksDocumentHolder;
use crate::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;
use crate::jwks_roller_server_error::JwksRollerServerError;
use crate::jwks_roller_service::JwksRollerService;
use crate::public_jwks_handler::PublicJwksHandler;

pub struct JwksRollerServerBundle {
    jwks_document_holder: JwksDocumentHolder,
    jwks_secret_holder: JwksSecretHolder,
    storage: Arc<dyn JwksSecretStorage>,
}

impl JwksRollerServerBundle {
    #[must_use]
    pub fn new(JwksRollerServerBundleParams { storage }: JwksRollerServerBundleParams) -> Self {
        Self {
            jwks_document_holder: JwksDocumentHolder::default(),
            jwks_secret_holder: JwksSecretHolder::default(),
            storage,
        }
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
        let rolled = roll(self.storage.as_ref(), &self.jwks_secret_holder, JWKS_CURVE)
            .map_err(JwksRollerServerError::SecretRoll)?;

        serde_json::to_vec(rolled.public_jwks())
            .map_err(JwksRollerServerError::DocumentSerialization)
            .map(|document| self.jwks_document_holder.set(Some(Bytes::from(document))))
    }
}

#[async_trait]
impl ServiceBundle for JwksRollerServerBundle {
    async fn services(self) -> Result<Vec<Box<dyn Service>>> {
        Ok(vec![Box::new(JwksRollerService { bundle: self })])
    }
}
