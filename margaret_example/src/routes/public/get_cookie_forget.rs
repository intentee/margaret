use margaret_cookie_jar::cookie_jar::CookieJar;
use margaret_cookie_jar::cookie_jar_error::CookieJarError;
use margaret_http::response::Response;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::cookie_name_visited::COOKIE_NAME_VISITED;

#[singleton]
#[responds_to_http(method = "get", path = "/cookie/forget", server = "public")]
pub struct GetCookieForget;

impl GetCookieForget {
    #[process]
    pub async fn respond(&self, cookies: &CookieJar) -> Response {
        match cookies.remove(COOKIE_NAME_VISITED) {
            Ok(()) => Response::text(200, "the visit cookie is cleared"),
            Err(CookieJarError::NotInRequest { .. }) => {
                Response::text(200, "there was no visit cookie to clear")
            }
            Err(_) => Response::text(500, "Internal Server Error"),
        }
    }
}
