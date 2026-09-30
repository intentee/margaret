use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_oidc_provider::revocation_endpoint::RevocationEndpoint;

use crate::form_of::form_of;

pub struct RevocationRoute {
    pub endpoint: Arc<RevocationEndpoint>,
}

#[async_trait]
impl Handler for RevocationRoute {
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::Done(
            self.endpoint
                .respond(request, form_of(request, body).await)
                .await
                .expect("the revocation endpoint reaches its state"),
        ))
    }
}
