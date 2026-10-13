use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::serves_session_endpoint;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::sessions::session_endpoint::SessionEndpoint;

#[responds_to_http(max_body_bytes = 4_096, method = RouteMethod::Post, path = "/sessions/refresh", server = "internal")]
#[serves_session_endpoint(SessionEndpoint::Refresh)]
pub struct PostSessionRefresh;
