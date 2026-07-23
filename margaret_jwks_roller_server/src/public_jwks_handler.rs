use async_trait::async_trait;

use margaret_http::handler::Handler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

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
        match self.jwks_document_holder.get() {
            None => Response::text(503, "The jwks document is not published yet"),
            Some(document) => Response::bytes(200, JWKS_CONTENT_TYPE, document),
        }
    }
}

#[async_trait]
impl Handler for PublicJwksHandler {
    async fn handle(&self, _request: &Request) -> ResponseContinuation {
        self.respond().into()
    }
}
