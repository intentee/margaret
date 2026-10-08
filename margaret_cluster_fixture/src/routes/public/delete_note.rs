use std::sync::Arc;

use margaret::framework::active_record::deletion::Deletion;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::models::note::Note;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Delete,
    name = "delete_note",
    path = "/notes/{note}",
    server = "public",
)]
pub struct DeleteNote {
    database: Arc<Database>,
}

impl DeleteNote {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
    }

    /// # Errors
    ///
    /// Returns an error when the note cannot be deleted.
    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "note")] note: Note,
    ) -> anyhow::Result<Response> {
        Ok(match note.delete(self.database.as_ref()).await? {
            Deletion::Deleted => Response::text(200, "deleted"),
            Deletion::Missing => Response::not_found(),
        })
    }
}
