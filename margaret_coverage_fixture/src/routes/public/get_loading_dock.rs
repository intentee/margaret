use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::auth::authenticated_users::courier::Courier;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/loading-dock", server = "public")]
pub struct GetLoadingDock;

impl GetLoadingDock {
    /// # Errors
    ///
    /// Never returns an error; every visitor is answered.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] courier: Option<Courier>,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(
            200,
            match courier {
                Some(Courier { name }) => format!("dock assigned to {name}"),
                None => "dock closed".to_string(),
            },
        ))
    }
}
