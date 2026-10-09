use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::serves_oidc_endpoint;
use margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint;
use margaret::framework::route_method::route_method::RouteMethod;

#[responds_to_http(
    method = RouteMethod::Get,
    name = "get_discovery",
    path = "/.well-known/openid-configuration",
    server = "public"
)]
#[serves_oidc_endpoint(OidcEndpoint::Discovery)]
pub struct GetDiscovery;
