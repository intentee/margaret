use std::sync::Arc;

use margaret_http::handler::Handler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;

pub struct FixtureIssuerRoutes {
    pub discovery: Arc<dyn Handler>,
    pub key_set: Arc<dyn Handler>,
}

impl FixtureIssuerRoutes {
    #[must_use]
    pub fn into_route_entries(self) -> Vec<RouteEntry> {
        vec![
            RouteEntry::new(
                "/.well-known/openid-configuration",
                vec![MethodHandler::anonymous("GET", self.discovery)],
            ),
            RouteEntry::new("/jwks", vec![MethodHandler::anonymous("GET", self.key_set)]),
        ]
    }
}
