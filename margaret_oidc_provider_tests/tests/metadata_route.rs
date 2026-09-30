use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_oidc_provider::provider_metadata_handler::ProviderMetadataHandler;

pub struct MetadataRoute {
    pub handler: Arc<ProviderMetadataHandler>,
}

#[async_trait]
impl Handler for MetadataRoute {
    async fn handle(
        &self,
        _request: &Request,
        _body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::Done(self.handler.respond()))
    }
}
