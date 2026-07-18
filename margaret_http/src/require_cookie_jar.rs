use margaret_cookie_jar::cookie_jar::CookieJar;
use margaret_cookie_jar::request_cookies::RequestCookies;

use crate::request::Request;
use crate::response::Response;

pub fn require_cookie_jar(request: &Request) -> Result<&CookieJar, Response> {
    match request.cookies() {
        RequestCookies::Present { cookie_jar } => Ok(cookie_jar),
        RequestCookies::Absent => Err(Response::text(500, "Internal Server Error")),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use http::HeaderMap;
    use http::Method;

    use margaret_cookie_jar::cookie_config::CookieConfig;
    use margaret_cookie_jar::cookie_domain::CookieDomain;
    use margaret_cookie_jar::cookie_jar::CookieJar;
    use margaret_cookie_jar::request_cookies::RequestCookies;

    use super::require_cookie_jar;
    use crate::request::Request;

    fn request_with_cookies(cookies: RequestCookies) -> Request {
        Request::new(Method::GET, "/".to_string()).with_cookies(Arc::new(cookies))
    }

    #[test]
    fn yields_the_jar_of_a_server_that_configured_cookies() {
        let cookie_config = Arc::new(CookieConfig {
            domain: CookieDomain::parse("example.test").expect("the domain parses"),
            secure: true,
        });
        let request = request_with_cookies(RequestCookies::Present {
            cookie_jar: CookieJar::from_headers(cookie_config, &HeaderMap::new())
                .expect("an empty header map parses"),
        });

        assert_eq!(
            require_cookie_jar(&request)
                .expect("the jar is available")
                .set_cookie_values(),
            Vec::<String>::new()
        );
    }

    #[test]
    fn refuses_a_server_that_configured_no_cookies() {
        let request = request_with_cookies(RequestCookies::Absent);

        assert_eq!(
            require_cookie_jar(&request)
                .expect_err("a server without cookies has no jar")
                .status(),
            500
        );
    }
}
