use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::forms::reader_preferences_cookie::ReaderPreferencesCookie;

#[singleton]
#[responds_to_http(method = "get", path = "/preferences", server = "public")]
pub struct GetPreferences;

impl GetPreferences {
    #[process]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn respond(
        &self,
        #[form_request(from = Cookie)] ReaderPreferencesCookie { theme }: ReaderPreferencesCookie,
    ) -> anyhow::Result<Response> {
        Ok({
            match theme {
                Some(theme) => Response::text(200, format!("theme: {theme}")),
                None => Response::text(200, "no theme preference set".to_string()),
            }
        })
    }
}
