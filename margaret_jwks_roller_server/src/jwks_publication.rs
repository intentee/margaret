use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;

use crate::jwks_document_holder::JwksDocumentHolder;
use crate::public_jwks_handler::PublicJwksHandler;

pub struct JwksPublication {
    jwks_document_holder: JwksDocumentHolder,
    jwks_secret_holder: JwksSecretHolder,
    public_jwks_handler: Arc<PublicJwksHandler>,
}

impl JwksPublication {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
    pub fn public_jwks_handler(&self) -> &PublicJwksHandler {
        &self.public_jwks_handler
    }
}

impl Default for JwksPublication {
    fn default() -> Self {
        let jwks_document_holder = JwksDocumentHolder::default();
        let public_jwks_handler = Arc::new(PublicJwksHandler::new(jwks_document_holder.clone()));

        Self {
            jwks_document_holder,
            jwks_secret_holder: JwksSecretHolder::default(),
            public_jwks_handler,
        }
    }
}
