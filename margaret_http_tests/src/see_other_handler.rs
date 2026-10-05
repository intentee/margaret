use async_trait::async_trait;

use margaret_http::forwardable_route::ForwardableRoute;
use margaret_http::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;

pub struct SeeOtherHandler {
    pub route: ForwardableRoute,
}

#[async_trait]
impl HeadHandler for SeeOtherHandler {
    async fn handle(&self, _request: &Request) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::from(self.route.see_other()))
    }
}
