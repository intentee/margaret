use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::serves_session_endpoint;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::sessions::session_endpoint::SessionEndpoint;

use crate::routes::public::get_notes::GetNotes;

#[responds_to_http(
    method = RouteMethod::Post,
    name = "post_staff_sign_out",
    path = "/staff/sign-out",
    server = "identity"
)]
#[serves_session_endpoint(SessionEndpoint::SignOut(landing_route = GetNotes))]
pub struct PostStaffSignOut;
