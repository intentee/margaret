use std::collections::HashMap;

use async_trait::async_trait;

use margaret_http::forward::Forward;
use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response_continuation::ResponseContinuation;

pub struct ForwardingHandler {
    pub path_params: HashMap<String, String>,
    pub target: &'static str,
}

#[async_trait]
impl Handler for ForwardingHandler {
    async fn handle(
        &self,
        _request: &Request,
        _body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::from(Forward::new(
            self.target,
            self.path_params.clone(),
        )))
    }
}
