use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::serves_session_endpoint;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::sessions::session_endpoint::SessionEndpoint;

use crate::get_profile::GetProfile;

#[responds_to_http(method = RouteMethod::Post, path = "/staff/sign-out", server = "staff")]
#[serves_session_endpoint(SessionEndpoint::SignOut(landing_route = GetProfile))]
pub struct PostStaffSignOut;
