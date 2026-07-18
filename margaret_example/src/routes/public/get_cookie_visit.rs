use cookie::Cookie;
use cookie::SameSite;

use margaret_cookie_jar::cookie_jar::CookieJar;
use margaret_http::response::Response;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::cookie_name_visited::COOKIE_NAME_VISITED;

fn visited_cookie() -> Cookie<'static> {
    Cookie::build((COOKIE_NAME_VISITED, "true"))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Strict)
        .secure(true)
        .build()
}

#[singleton]
#[responds_to_http(method = "get", path = "/cookie/visit", server = "public")]
pub struct GetCookieVisit;

impl GetCookieVisit {
    #[process]
    pub async fn respond(&self, cookies: &CookieJar) -> Response {
        let greeting = match cookies.get(COOKIE_NAME_VISITED) {
            Some(_) => "welcome back",
            None => "nice to meet you",
        };

        match cookies.add(visited_cookie()) {
            Ok(()) => Response::text(200, greeting),
            Err(_) => Response::text(500, "Internal Server Error"),
        }
    }
}
