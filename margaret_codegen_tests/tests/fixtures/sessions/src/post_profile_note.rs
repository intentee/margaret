use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::profile_note::ProfileNote;
use crate::reader::Reader;

#[singleton]
#[responds_to_http(
    max_body_bytes = 1_024,
    method = RouteMethod::Post,
    name = "post_profile_note",
    path = "/profile/notes",
    server = "public"
)]
pub struct PostProfileNote;

impl PostProfileNote {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] Reader { subject }: Reader,
        #[form_request(from = RequestInput::Form)] ProfileNote { text }: ProfileNote,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(201, format!("{subject}: {text}")))
    }
}
