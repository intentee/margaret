use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::route_models::shelf::Shelf;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/shelves/{shelf}", server = "public")]
pub struct GetShelf;

impl GetShelf {
    /// # Errors
    ///
    /// Never returns an error; every bound shelf is described.
    #[process]
    pub fn respond(
        &self,
        #[route_parameter(from = "shelf")] Shelf { name }: Shelf,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(200, format!("shelf {name}")))
    }
}
