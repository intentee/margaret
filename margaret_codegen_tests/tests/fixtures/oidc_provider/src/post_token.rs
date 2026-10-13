use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::serves_oidc_endpoint;
use margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint;
use margaret::framework::route_method::route_method::RouteMethod;

#[responds_to_http(max_body_bytes = 16384, method = RouteMethod::Post, path = "/token", server = "public")]
#[serves_oidc_endpoint(OidcEndpoint::Token)]
pub struct PostToken;
