use margaret::framework::http::forward::Forward;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::url_path::path_parameter_value::PathParameterValue;

use crate::margaret::routes::Routes;
use crate::margaret::routes::forward_targets::PublicForwardTarget;
use crate::margaret::routes::props::GetNoteRouteProps;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Get,
    name = "get_forwarded_note",
    path = "/forwarded-notes/{note_id}",
    server = "public",
)]
pub struct GetForwardedNote;

impl GetForwardedNote {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        routes: &Routes,
        #[route_parameter(from = "note_id")] note_id: String,
    ) -> anyhow::Result<Forward<PublicForwardTarget>> {
        PathParameterValue::new(note_id)
            .map(|note| {
                routes
                    .public
                    .get_note
                    .forward_to(GetNoteRouteProps { note })
            })
            .map_err(anyhow::Error::from)
    }
}
