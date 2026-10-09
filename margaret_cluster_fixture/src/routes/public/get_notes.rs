use std::sync::Arc;

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
    method = RouteMethod::Get,
    name = "get_notes",
    path = "/notes",
    server = "public",
)]
pub struct GetNotes {
    database: Arc<Database>,
}

impl GetNotes {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
    }

    /// # Errors
    ///
    /// Returns an error when the notes cannot be read.
    #[process]
    pub async fn respond(&self) -> anyhow::Result<Response> {
        Ok(Response::json(
            200,
            &Note::in_id_order(self.database.as_ref()).await?,
        ))
    }
}
