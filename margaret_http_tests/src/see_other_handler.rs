use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::get_target::GetTarget;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::route_reference::RouteReference;

pub struct SeeOtherHandler {
    pub route: RouteReference<GetTarget>,
}

#[async_trait]
impl HeadHandler for SeeOtherHandler {
    async fn handle(&self, _request: &Request) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::from(self.route.see_other()))
    }
}
