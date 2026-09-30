use async_trait::async_trait;

use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

pub struct PathParamEchoHandler {
    pub name: &'static str,
}

#[async_trait]
impl Handler for PathParamEchoHandler {
    async fn handle(
        &self,
        request: &Request,
        _body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::Done(
            match request.path_param(self.name) {
                Some(value) => Response::text(200, value.to_string()),
                None => Response::not_found(),
            },
        ))
    }
}
