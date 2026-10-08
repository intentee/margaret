use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::models::note::Note;
use crate::stores::note_store::NoteStore;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Delete,
    name = "delete_note",
    path = "/notes/{note}",
    server = "public",
)]
pub struct DeleteNote {
    notes: Arc<NoteStore>,
}

impl DeleteNote {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(notes: Arc<NoteStore>) -> anyhow::Result<Self> {
        Ok(Self { notes })
    }

    /// # Errors
    ///
    /// Returns an error when the note cannot be deleted.
    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "note")] Note { id, .. }: Note,
    ) -> anyhow::Result<Response> {
        self.notes.delete(id).await?;

        Ok(Response::text(200, "deleted"))
    }
}
