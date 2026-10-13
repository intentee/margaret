use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::serves_session_endpoint;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::sessions::session_endpoint::SessionEndpoint;

use crate::routes::public::get_health::GetHealth;

#[responds_to_http(
    method = RouteMethod::Post,
    name = "post_sign_out",
    path = "/sign-out",
    server = "public"
)]
#[serves_session_endpoint(SessionEndpoint::SignOut(landing_route = GetHealth))]
pub struct PostSignOut;
