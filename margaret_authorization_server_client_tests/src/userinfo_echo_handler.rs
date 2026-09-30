use async_trait::async_trait;
use http::header::AUTHORIZATION;
use serde_json::json;

use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

pub struct UserinfoEchoHandler;

#[async_trait]
impl Handler for UserinfoEchoHandler {
    async fn handle(
        &self,
        request: &Request,
        _body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::Done(Response::json(
            200,
            &json!({
                "authorization": request.inputs.server.header(&AUTHORIZATION),
                "sub": "subject",
            }),
        )))
    }
}
