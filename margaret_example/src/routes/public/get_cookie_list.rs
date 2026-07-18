use margaret_cookie_jar::cookie_jar::CookieJar;
use margaret_http::response::Response;
use margaret_macros::middleware;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[middleware(cookie_audited)]
#[responds_to_http(method = "get", path = "/cookie/list", server = "public")]
pub struct GetCookieList;

impl GetCookieList {
    #[process]
    pub async fn respond(&self, cookies: &CookieJar) -> Response {
        let mut listed: Vec<String> = cookies
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect();

        listed.sort();

        Response::text(200, listed.join("\n"))
    }
}
