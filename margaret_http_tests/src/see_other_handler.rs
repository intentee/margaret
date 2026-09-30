use async_trait::async_trait;

use margaret_http::forwardable_route::ForwardableRoute;
use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response_continuation::ResponseContinuation;

pub struct SeeOtherHandler {
    pub route: ForwardableRoute,
}

#[async_trait]
impl Handler for SeeOtherHandler {
    async fn handle(
        &self,
        _request: &Request,
        _body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::from(self.route.see_other()))
    }
}
