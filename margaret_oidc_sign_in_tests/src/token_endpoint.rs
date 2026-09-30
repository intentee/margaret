use std::sync::OnceLock;

use async_trait::async_trait;

use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

use crate::token_answer::TokenAnswer;

pub struct TokenEndpoint {
    pub answer: OnceLock<TokenAnswer>,
}

#[async_trait]
impl Handler for TokenEndpoint {
    async fn handle(
        &self,
        _request: &Request,
        _body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        let TokenAnswer { body, status } = self.answer.wait();

        Ok(ResponseContinuation::Done(Response::json(*status, body)))
    }
}
