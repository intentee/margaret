use std::sync::Arc;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::database::database::Database;
use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::validation::validation_result::ValidationResult;

use crate::forms::note_form::NoteForm;
use crate::margaret::models::models_note_note::draft::Draft;
use crate::models::note::Note;

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = RouteMethod::Post,
    name = "post_note",
    path = "/notes",
    server = "public",
)]
pub struct PostNote {
    database: Arc<Database>,
}

impl PostNote {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
    }

    /// # Errors
    ///
    /// Returns an error when the note cannot be stored.
    #[process]
    pub async fn respond(
        &self,
        #[form_request(from = RequestInput::Json)] form: ValidationResult<NoteForm>,
    ) -> anyhow::Result<Response> {
        Ok(match form {
            ValidationResult::Valid(NoteForm { body, expires_at }) => Response::json(
                201,
                &Note::create(Draft { body, expires_at })
                    .run(self.database.as_ref())
                    .await?,
            ),
            ValidationResult::Invalid(errors) => Response::text(422, errors.to_string()),
            ValidationResult::Malformed(malformation) => {
                Response::text(400, malformation.to_string())
            }
        })
    }
}
