use bytes::Bytes;
use cookie::Cookie;
use http_body_util::Full;

use crate::header::Header;

pub struct Response {
    body: String,
    headers: Vec<Header>,
    status: u16,
}

impl Response {
    pub fn text(status: u16, body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            headers: Vec::new(),
            status,
        }
    }

    pub fn html(status: u16, body: impl Into<String>) -> Self {
        Self::text(status, body).header("content-type", "text/html; charset=utf-8")
    }

    pub fn see_other(location: impl Into<String>) -> Self {
        Self::text(303, "").header("location", location)
    }

    pub fn not_found() -> Self {
        Self::text(404, "Not Found")
    }

    pub fn forbidden() -> Self {
        Self::text(403, "Forbidden")
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push(Header {
            name: name.into(),
            value: value.into(),
        });

        self
    }

    pub fn set_cookie(self, cookie: Cookie<'static>) -> Self {
        self.header("set-cookie", cookie.to_string())
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub(crate) fn into_http(self) -> http::Response<Full<Bytes>> {
        let mut builder = http::Response::builder().status(self.status);

        for header in self.headers {
            builder = builder.header(header.name, header.value);
        }

        builder
            .body(Full::new(Bytes::from(self.body)))
            .expect("a well-formed response is built")
    }
}

#[cfg(test)]
mod tests {
    use cookie::Cookie;

    use super::Response;

    #[test]
    fn builds_an_http_response_with_status_and_headers() {
        let response = Response::text(201, "created")
            .header("x-marker", "on")
            .into_http();

        assert_eq!(response.status().as_u16(), 201);
        assert!(response.headers().contains_key("x-marker"));
    }

    #[test]
    fn builds_an_html_response_with_a_content_type() {
        let response = Response::html(200, "<p>hi</p>").into_http();

        assert_eq!(
            response
                .headers()
                .get("content-type")
                .expect("the content type header is present"),
            "text/html; charset=utf-8"
        );
    }

    #[test]
    fn builds_a_see_other_redirect_with_a_location() {
        let response = Response::see_other("/profile").into_http();

        assert_eq!(response.status().as_u16(), 303);
        assert_eq!(
            response
                .headers()
                .get("location")
                .expect("the location header is present"),
            "/profile"
        );
    }

    #[test]
    fn attaches_a_set_cookie_header() {
        let response = Response::see_other("/profile")
            .set_cookie(Cookie::new("session", "abc"))
            .into_http();

        assert!(
            response
                .headers()
                .get("set-cookie")
                .expect("the set-cookie header is present")
                .to_str()
                .expect("the header is valid text")
                .starts_with("session=abc")
        );
    }
}
