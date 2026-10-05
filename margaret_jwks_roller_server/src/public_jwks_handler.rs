use margaret_http::response::Response;

use crate::jwks_document_holder::JwksDocumentHolder;

const JWKS_CONTENT_TYPE: &str = "application/jwk-set+json";

pub struct PublicJwksHandler {
    jwks_document_holder: JwksDocumentHolder,
}

impl PublicJwksHandler {
    #[must_use]
    pub fn new(jwks_document_holder: JwksDocumentHolder) -> Self {
        Self {
            jwks_document_holder,
        }
    }

    #[must_use]
    pub fn respond(&self) -> Response {
        Response::bytes(200, JWKS_CONTENT_TYPE, self.jwks_document_holder.get())
    }
}
