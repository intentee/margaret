use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::auth::authenticated_users::patron::Patron;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/reading-room", server = "public")]
pub struct GetReadingRoom;

impl GetReadingRoom {
    /// # Errors
    ///
    /// Never returns an error; every visitor is answered.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] patron: Option<Patron>,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(
            200,
            match patron {
                Some(Patron { card }) => format!("reserved seat for card {card}"),
                None => "open seating".to_string(),
            },
        ))
    }
}
