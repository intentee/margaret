use async_trait::async_trait;
use cookie::Cookie;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::cookie_changes::CookieChanges;
use margaret_http::forwardable_route::ForwardableRoute;
use margaret_http::get_route_reference::GetRouteReference;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;

use crate::fixture_forward_target::FixtureForwardTarget;
use crate::target_route_props::TargetRouteProps;

pub struct CookieChangingForwardHandler {
    pub cookie: Cookie<'static>,
    pub route: ForwardableRoute<FixtureForwardTarget, TargetRouteProps, GetRouteReference>,
}

#[async_trait]
impl HeadHandler<FixtureForwardTarget> for CookieChangingForwardHandler {
    async fn handle(
        &self,
        _request: &Request,
    ) -> Result<ResponseContinuation<FixtureForwardTarget>, HandlerError> {
        Ok(CookieChanges {
            cookies: vec![self.cookie.clone()],
        }
        .precede(ResponseContinuation::from(
            self.route.forward_to(TargetRouteProps),
        )))
    }
}
