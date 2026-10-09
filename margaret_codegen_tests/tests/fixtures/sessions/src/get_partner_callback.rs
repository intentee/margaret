use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::serves_sign_in;
use margaret::framework::oidc_sign_in::sign_in_endpoint::SignInEndpoint;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::get_welcome::GetWelcome;

#[responds_to_http(method = RouteMethod::Get, path = "/partner/callback", server = "public")]
#[serves_sign_in(SignInEndpoint::Callback(landing_route = GetWelcome), client = partner_client)]
pub struct GetPartnerCallback;
