use margaret::framework::http::forward::Forward;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::forwarders::public::Forwarder;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Get,
    path = "/forwarded-notes/{note_id}",
    server = "public"
)]
pub struct GetForwardedNote;

impl GetForwardedNote {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        forward: Forwarder,
        #[route_parameter(from = "note_id")] note_id: String,
    ) -> anyhow::Result<Forward> {
        Ok(forward.get_note(note_id))
    }
}
