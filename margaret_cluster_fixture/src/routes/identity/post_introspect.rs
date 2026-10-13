use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::serves_oidc_endpoint;
use margaret::framework::oidc_provider::oidc_endpoint::OidcEndpoint;
use margaret::framework::route_method::route_method::RouteMethod;

#[responds_to_http(max_body_bytes = 8_388_608, method = RouteMethod::Post, path = "/introspect", server = "identity")]
#[serves_oidc_endpoint(OidcEndpoint::Introspection)]
pub struct PostIntrospect;
