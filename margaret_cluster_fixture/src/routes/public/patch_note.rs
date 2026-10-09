use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::validation::validation_result::ValidationResult;

use crate::forms::note_form::NoteForm;
use crate::models::note::Note;
use crate::stores::note_store::NoteStore;

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = RouteMethod::Patch,
    name = "patch_note",
    path = "/notes/{note}",
    server = "public",
)]
pub struct PatchNote {
    notes: Arc<NoteStore>,
}

impl PatchNote {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(notes: Arc<NoteStore>) -> anyhow::Result<Self> {
        Ok(Self { notes })
    }

    /// # Errors
    ///
    /// Returns an error when the note cannot be updated.
    #[process]
    pub async fn respond(
        &self,
        #[route_parameter(from = "note")] Note { id, .. }: Note,
        #[form_request(from = RequestInput::Json)] form: ValidationResult<NoteForm>,
    ) -> anyhow::Result<Response> {
        Ok(match form {
            ValidationResult::Valid(NoteForm { body, .. }) => {
                self.notes.update(id, &body).await?;

                Response::text(200, "updated")
            }
            ValidationResult::Invalid(errors) => Response::text(422, errors.to_string()),
            ValidationResult::Malformed(malformation) => {
                Response::text(400, malformation.to_string())
            }
        })
    }
}
