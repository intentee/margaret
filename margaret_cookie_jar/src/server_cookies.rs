use std::sync::Arc;

use http::HeaderMap;

use crate::cookie_config::CookieConfig;
use crate::cookie_jar::CookieJar;
use crate::cookie_jar_error::CookieJarError;
use crate::request_cookies::RequestCookies;

pub enum ServerCookies {
    Absent,
    Present { cookie_config: Arc<CookieConfig> },
}

impl ServerCookies {
    pub fn stage(&self, headers: &HeaderMap) -> Result<RequestCookies, CookieJarError> {
        match self {
            Self::Absent => Ok(RequestCookies::Absent),
            Self::Present { cookie_config } => Ok(RequestCookies::Present {
                cookie_jar: CookieJar::from_headers(cookie_config.clone(), headers)?,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use http::HeaderMap;
    use http::HeaderValue;
    use http::header::COOKIE;

    use super::ServerCookies;
    use crate::cookie_config::CookieConfig;
    use crate::cookie_domain::CookieDomain;
    use crate::request_cookies::RequestCookies;

    fn present() -> ServerCookies {
        ServerCookies::Present {
            cookie_config: Arc::new(CookieConfig {
                domain: CookieDomain::parse("example.test").expect("the domain parses"),
                secure: true,
            }),
        }
    }

    #[test]
    fn builds_no_cookie_jar_for_a_server_without_cookies() {
        let mut headers = HeaderMap::new();

        headers.insert(COOKIE, HeaderValue::from_static("session=abc"));

        assert_eq!(
            format!(
                "{:?}",
                ServerCookies::Absent
                    .stage(&headers)
                    .expect("a cookieless server stages without reading the header")
            ),
            "Absent"
        );
    }

    #[test]
    fn builds_a_cookie_jar_seeded_from_the_request() {
        let mut headers = HeaderMap::new();

        headers.insert(COOKIE, HeaderValue::from_static("session=abc"));

        assert!(matches!(
            present()
                .stage(&headers)
                .expect("a cookie server stages a jar"),
            RequestCookies::Present { ref cookie_jar }
                if cookie_jar.get("session") == Some("abc".to_owned())
        ));
    }

    #[test]
    fn propagates_a_malformed_cookie_header() {
        let mut headers = HeaderMap::new();

        headers.insert(COOKIE, HeaderValue::from_static("session=a; session=b"));

        assert!(
            present()
                .stage(&headers)
                .expect_err("a repeated cookie name is rejected")
                .to_string()
                .contains("more than once")
        );
    }
}
