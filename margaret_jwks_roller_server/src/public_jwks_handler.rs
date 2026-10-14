use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::handles_head::HandlesHead;
use margaret_http::request::Request;
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
}

#[async_trait]
impl HandlesHead for PublicJwksHandler {
    async fn handle(&self, _request: &Request) -> Result<Response, HandlerError> {
        Ok(Response::bytes(
            200,
            JWKS_CONTENT_TYPE,
            self.jwks_document_holder.get(),
        ))
    }
}
