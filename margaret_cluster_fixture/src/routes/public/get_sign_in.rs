use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::serves_sign_in;
use margaret::framework::oidc_sign_in::sign_in_endpoint::SignInEndpoint;
use margaret::framework::route_method::route_method::RouteMethod;

#[responds_to_http(
    method = RouteMethod::Get,
    name = "get_sign_in",
    path = "/sign-in",
    server = "public",
)]
#[serves_sign_in(SignInEndpoint::Start, client = cluster)]
pub struct GetSignIn;
