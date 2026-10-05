use std::sync::Arc;

use margaret_http::head_handler::HeadHandler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_route_method::route_method::RouteMethod;

pub struct FixtureIssuerRoutes {
    pub discovery: Arc<dyn HeadHandler>,
    pub key_set: Arc<dyn HeadHandler>,
}

impl FixtureIssuerRoutes {
    #[must_use]
    pub fn into_route_entries(self) -> Vec<RouteEntry> {
        vec![
            RouteEntry::new(
                "/.well-known/openid-configuration",
                vec![MethodHandler::head(RouteMethod::Get, self.discovery)],
            ),
            RouteEntry::new(
                "/jwks",
                vec![MethodHandler::head(RouteMethod::Get, self.key_set)],
            ),
        ]
    }
}
