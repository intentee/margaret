use std::convert::Infallible;
use std::sync::Arc;

use margaret_http::head_handler::HeadHandler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_route_method::route_method::RouteMethod;

pub struct FixtureIssuerRoutes {
    pub discovery: Arc<dyn HeadHandler<Infallible>>,
    pub key_set: Arc<dyn HeadHandler<Infallible>>,
}

impl FixtureIssuerRoutes {
    pub const PATHS: [&'static str; 2] = ["/.well-known/openid-configuration", "/jwks"];

    #[must_use]
    pub fn into_route_entries(self) -> [RouteEntry<Infallible>; 2] {
        [
            RouteEntry::new(vec![MethodHandler::head(RouteMethod::Get, self.discovery)]),
            RouteEntry::new(vec![MethodHandler::head(RouteMethod::Get, self.key_set)]),
        ]
    }
}
