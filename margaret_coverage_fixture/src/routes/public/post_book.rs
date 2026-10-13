use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::forms::book_form::BookForm;

#[singleton]
#[responds_to_http(
    max_body_bytes = 1_024,
    method = RouteMethod::Post,
    path = "/books",
    server = "public"
)]
pub struct PostBook;

impl PostBook {
    /// # Errors
    ///
    /// Never returns an error; every validated book is catalogued.
    #[process]
    pub fn respond(
        &self,
        #[form_request(from = RequestInput::Form)] BookForm { title }: BookForm,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(201, format!("catalogued {title}")))
    }
}
