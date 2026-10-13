use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::serves_oidc_endpoint;
use margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint;
use margaret::framework::route_method::route_method::RouteMethod;

#[responds_to_http(method = RouteMethod::Get, path = "/.well-known/openid-configuration", server = "identity")]
#[serves_oidc_endpoint(OidcEndpoint::Discovery)]
pub struct GetDiscovery;
