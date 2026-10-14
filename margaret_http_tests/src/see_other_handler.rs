use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::get_route_reference::GetRouteReference;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;

pub struct SeeOtherHandler {
    pub route: GetRouteReference,
}

#[async_trait]
impl<TTarget> HeadHandler<TTarget> for SeeOtherHandler {
    async fn handle(
        &self,
        _request: &Request,
    ) -> Result<ResponseContinuation<TTarget>, HandlerError> {
        Ok(ResponseContinuation::from(self.route.see_other()))
    }
}
