use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::forms::reader_preferences_cookie::ReaderPreferencesCookie;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/preferences", server = "public")]
pub struct GetPreferences;

impl GetPreferences {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[form_request(from = RequestInput::Cookie)]
        ReaderPreferencesCookie { theme }: ReaderPreferencesCookie,
    ) -> anyhow::Result<Response> {
        Ok({
            match theme {
                Some(theme) => Response::text(200, format!("theme: {theme}")),
                None => Response::text(200, "no theme preference set".to_string()),
            }
        })
    }
}
