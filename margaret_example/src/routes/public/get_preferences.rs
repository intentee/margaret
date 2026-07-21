use margaret_http::response::Response;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::forms::reader_preferences_cookie::ReaderPreferencesCookie;

#[singleton]
#[responds_to_http(method = "get", path = "/preferences", server = "public")]
pub struct GetPreferences;

impl GetPreferences {
    #[process]
    pub async fn respond(
        &self,
        #[form_request(from = Cookie)] ReaderPreferencesCookie { theme }: ReaderPreferencesCookie,
    ) -> Response {
        match theme {
            Some(theme) => Response::text(200, format!("theme: {theme}")),
            None => Response::text(200, "no theme preference set".to_string()),
        }
    }
}
