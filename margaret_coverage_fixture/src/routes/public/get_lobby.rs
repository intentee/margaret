use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::auth::authenticated_users::visitor::Visitor;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/lobby", server = "public")]
pub struct GetLobby;

impl GetLobby {
    /// # Errors
    ///
    /// Never returns an error; every visitor is answered.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] visitor: Option<Visitor>,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(
            200,
            match visitor {
                Some(Visitor { name }) => format!("welcome, {name}"),
                None => "welcome".to_string(),
            },
        ))
    }
}
