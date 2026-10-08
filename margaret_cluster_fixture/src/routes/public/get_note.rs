use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::models::note::Note;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Get,
    name = "get_note",
    path = "/notes/{note}",
    server = "public"
)]
pub struct GetNote;

impl GetNote {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[route_parameter(from = "note")] note: Note,
    ) -> anyhow::Result<Response> {
        Ok(Response::json(200, &note))
    }
}
