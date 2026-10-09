use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

pub struct PathParamEchoHandler {
    pub name: &'static str,
}

#[async_trait]
impl HeadHandler for PathParamEchoHandler {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::Done(
            match request.path_param(self.name) {
                Some(value) => Response::text(200, value.to_string()),
                None => Response::not_found(),
            },
        ))
    }
}
