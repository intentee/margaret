use std::convert::Infallible;
use std::error::Error;
use std::io;

use bytes::Bytes;
use cookie::Cookie;
use futures_util::Stream;
use futures_util::TryStreamExt;
use http::HeaderName;
use http::StatusCode;
use http::header::SET_COOKIE;
use http_body::Frame;
use http_body_util::BodyExt;
use http_body_util::Full;
use http_body_util::StreamBody;
use http_body_util::combinators::UnsyncBoxBody;
use maud::Markup;
use serde::Serialize;

use crate::header::Header;

fn buffered(body: Bytes) -> UnsyncBoxBody<Bytes, io::Error> {
    Full::new(body)
        .map_err(|never: Infallible| match never {})
        .boxed_unsync()
}

fn set_cookie_header(cookie: &Cookie<'_>) -> Header {
    Header {
        name: SET_COOKIE.as_str().to_string(),
        value: cookie.to_string(),
    }
}

fn internal_server_error() -> http::Response<UnsyncBoxBody<Bytes, io::Error>> {
    let mut response = http::Response::new(buffered(Bytes::from_static(b"Internal Server Error")));

    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

    response
}

pub struct Response {
    body: UnsyncBoxBody<Bytes, io::Error>,
    headers: Vec<Header>,
    status: u16,
}

impl Response {
    pub fn bytes(status: u16, content_type: impl Into<String>, body: impl Into<Bytes>) -> Self {
        Self::new(status, buffered(body.into())).header("content-type", content_type)
    }

    #[must_use]
    pub fn forbidden() -> Self {
        Self::text(403, "Forbidden")
    }

    pub fn html(status: u16, body: impl Into<String>) -> Self {
        Self::bytes(status, "text/html; charset=utf-8", body.into())
    }

    pub fn json<Value: Serialize>(status: u16, value: &Value) -> Self {
        Self::json_from_serialization(status, serde_json::to_string(value))
    }

    fn json_from_serialization(status: u16, serialized: Result<String, serde_json::Error>) -> Self {
        match serialized {
            Ok(body) => Self::bytes(status, "application/json", body),
            Err(error) => {
                eprintln!("margaret_http: response serialization failed: {error}");

                Self::text(500, "Internal Server Error")
            }
        }
    }

    fn new(status: u16, body: UnsyncBoxBody<Bytes, io::Error>) -> Self {
        Self {
            body,
            headers: Vec::new(),
            status,
        }
    }

    #[must_use]
    pub fn not_found() -> Self {
        Self::text(404, "Not Found")
    }

    #[must_use]
    pub fn static_bytes(status: u16, content_type: &'static str, body: &'static [u8]) -> Self {
        Self::new(status, buffered(Bytes::from_static(body))).header("content-type", content_type)
    }

    pub fn stream<TChunks, TError>(
        status: u16,
        content_type: impl Into<String>,
        chunks: TChunks,
    ) -> Self
    where
        TChunks: Stream<Item = Result<Bytes, TError>> + Send + 'static,
        TError: Into<Box<dyn Error + Send + Sync>> + 'static,
    {
        Self::new(
            status,
            StreamBody::new(chunks.map_ok(Frame::data).map_err(io::Error::other)).boxed_unsync(),
        )
        .header("content-type", content_type)
    }

    pub fn text(status: u16, body: impl Into<String>) -> Self {
        Self::new(status, buffered(Bytes::from(body.into())))
    }

    #[must_use]
    pub fn unauthorized() -> Self {
        Self::text(401, "Unauthorized")
    }

    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push(Header {
            name: name.into(),
            value: value.into(),
        });

        self
    }

    #[must_use]
    pub fn header_value(&self, name: &HeaderName) -> Option<&str> {
        self.headers
            .iter()
            .find(|header| header.name.eq_ignore_ascii_case(name.as_str()))
            .map(|header| header.value.as_str())
    }

    #[must_use]
    pub fn headers(&self) -> &[Header] {
        &self.headers
    }

    #[must_use]
    pub fn set_cookie(mut self, cookie: &Cookie<'_>) -> Self {
        self.headers.push(set_cookie_header(cookie));

        self
    }

    #[must_use]
    pub fn status(&self) -> u16 {
        self.status
    }

    pub(crate) fn into_http(self) -> http::Response<UnsyncBoxBody<Bytes, io::Error>> {
        let status = self.status;
        let mut builder = http::Response::builder().status(status);

        for header in self.headers {
            builder = builder.header(header.name, header.value);
        }

        match builder.body(self.body) {
            Ok(response) => response,
            Err(error) => {
                eprintln!(
                    "margaret_http: the responder produced a response that is not a valid HTTP response (status `{status}`): {error}"
                );

                internal_server_error()
            }
        }
    }

    pub(crate) fn preceded_by_cookies(mut self, cookies: &[Cookie<'_>]) -> Self {
        self.headers
            .splice(0..0, cookies.iter().map(set_cookie_header));

        self
    }
}

impl From<Markup> for Response {
    fn from(markup: Markup) -> Self {
        Self::html(200, markup.into_string())
    }
}

#[cfg(test)]
mod tests {
    use cookie::Cookie;
    use http::header::LOCATION;
    use http_body_util::BodyExt;
    use serde::Serialize;
    use serde::Serializer;
    use serde::ser::Error;

    use super::Response;

    const PNG_MAGIC: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0xFF];

    struct Unserializable;

    impl Serialize for Unserializable {
        fn serialize<Target: Serializer>(
            &self,
            _serializer: Target,
        ) -> Result<Target::Ok, Target::Error> {
            Err(Target::Error::custom("this value cannot be serialized"))
        }
    }

    #[tokio::test]
    async fn serves_a_binary_body_without_altering_it() {
        let body = Response::bytes(200, "image/png", PNG_MAGIC)
            .into_http()
            .into_body()
            .collect()
            .await
            .expect("the buffered body is collected")
            .to_bytes();

        assert_eq!(body.as_ref(), PNG_MAGIC);
    }

    #[test]
    fn builds_a_bytes_response_with_the_given_content_type() {
        let response = Response::bytes(200, "image/png", PNG_MAGIC).into_http();

        assert_eq!(
            response
                .headers()
                .get("content-type")
                .expect("the content type header is present"),
            "image/png"
        );
    }

    #[tokio::test]
    async fn serves_static_bytes_with_the_given_content_type() {
        let response = Response::static_bytes(200, "image/png", PNG_MAGIC).into_http();

        assert_eq!(response.status().as_u16(), 200);
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .expect("the content type header is present"),
            "image/png"
        );

        let body = response
            .into_body()
            .collect()
            .await
            .expect("the buffered body is collected")
            .to_bytes();

        assert_eq!(body.as_ref(), PNG_MAGIC);
    }

    #[test]
    fn builds_a_json_response_with_a_content_type() {
        let response = Response::json(200, &vec!["ok"]).into_http();

        assert_eq!(response.status().as_u16(), 200);
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .expect("the content type header is present"),
            "application/json"
        );
    }

    #[tokio::test]
    async fn serves_an_error_when_the_value_cannot_be_serialized() {
        let response = Response::json(200, &Unserializable).into_http();
        let status = response.status().as_u16();
        let body = response
            .into_body()
            .collect()
            .await
            .expect("the response body collects")
            .to_bytes();

        assert_eq!(status, 500);
        assert_eq!(body, "Internal Server Error");
    }

    #[test]
    fn builds_an_http_response_with_status_and_headers() {
        let response = Response::text(201, "created").header("x-marker", "on");

        assert_eq!(response.status(), 201);

        let http = response.into_http();

        assert_eq!(http.status().as_u16(), 201);
        assert!(http.headers().contains_key("x-marker"));
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
    fn builds_a_forbidden_response() {
        let response = Response::forbidden().into_http();

        assert_eq!(response.status().as_u16(), 403);
    }

    #[test]
    fn builds_an_unauthorized_response() {
        let response = Response::unauthorized().into_http();

        assert_eq!(response.status().as_u16(), 401);
    }

    #[test]
    fn serves_an_internal_server_error_when_the_response_is_not_a_valid_http_response() {
        let response = Response::text(9999, "unreachable status").into_http();

        assert_eq!(response.status().as_u16(), 500);
    }

    #[test]
    fn finds_a_header_value_regardless_of_the_case_of_its_name() {
        let response = Response::text(303, "").header("Location", "https://localhost/");

        assert_eq!(response.header_value(&LOCATION), Some("https://localhost/"));
    }

    #[test]
    fn finds_no_value_of_an_absent_header() {
        assert_eq!(Response::text(200, "").header_value(&LOCATION), None);
    }

    #[test]
    fn attaches_a_set_cookie_header() {
        let response = Response::text(200, "")
            .set_cookie(&Cookie::new("session", "abc"))
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

    #[tokio::test]
    async fn converts_maud_markup_into_an_html_response() {
        let response: Response = maud::html! { p { "hi" } }.into();
        let http = response.into_http();

        assert_eq!(http.status().as_u16(), 200);
        assert_eq!(
            http.headers()
                .get("content-type")
                .expect("the content type header is present"),
            "text/html; charset=utf-8"
        );

        let body = http
            .into_body()
            .collect()
            .await
            .expect("the body collects")
            .to_bytes();

        assert_eq!(body.as_ref(), b"<p>hi</p>");
    }

    #[test]
    fn builds_an_html_response_from_markup_directly() {
        let response = Response::html(201, maud::html! { p { "hi" } });

        assert_eq!(response.status(), 201);
    }
}
