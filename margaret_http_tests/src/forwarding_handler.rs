use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::forwardable_route::ForwardableRoute;
use margaret_http::get_route_reference::GetRouteReference;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;

use crate::fixture_forward_target::FixtureForwardTarget;

pub struct ForwardingHandler<TProps> {
    pub props: fn() -> TProps,
    pub route: ForwardableRoute<FixtureForwardTarget, TProps, GetRouteReference>,
}

#[async_trait]
impl<TProps: 'static> HeadHandler<FixtureForwardTarget> for ForwardingHandler<TProps> {
    async fn handle(
        &self,
        _request: &Request,
    ) -> Result<ResponseContinuation<FixtureForwardTarget>, HandlerError> {
        Ok(ResponseContinuation::from(
            self.route.forward_to((self.props)()),
        ))
    }
}
