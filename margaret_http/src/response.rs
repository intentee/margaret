use bytes::Bytes;
use http::StatusCode;
use http_body_util::Full;
use serde::Serialize;

use crate::header::Header;

fn internal_server_error() -> http::Response<Full<Bytes>> {
    let mut response = http::Response::new(Full::new(Bytes::from_static(b"Internal Server Error")));

    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

    response
}

pub struct Response {
    body: String,
    headers: Vec<Header>,
    status: u16,
}

impl Response {
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

    pub fn html(status: u16, body: impl Into<String>) -> Self {
        Self::text(status, body).header("content-type", "text/html; charset=utf-8")
    }

    pub fn json<Value: Serialize>(status: u16, value: &Value) -> Self {
        match serde_json::to_string(value) {
            Ok(body) => Self::text(status, body).header("content-type", "application/json"),
            Err(error) => Self::text(500, error.to_string()),
        }
    }

    pub fn not_found() -> Self {
        Self::text(404, "Not Found")
    }

    pub fn text(status: u16, body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            headers: Vec::new(),
            status,
        }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub(crate) fn into_http(self) -> http::Response<Full<Bytes>> {
        let status = self.status;
        let mut builder = http::Response::builder().status(status);

        for header in self.headers {
            builder = builder.header(header.name, header.value);
        }

        match builder.body(Full::new(Bytes::from(self.body))) {
            Ok(response) => response,
            Err(error) => {
                eprintln!(
                    "margaret_http: the responder produced a response that is not a valid HTTP response (status `{status}`): {error}"
                );

                internal_server_error()
            }
        }
    }

    pub(crate) fn with_cookies(self, set_cookie_values: Vec<String>) -> Self {
        set_cookie_values
            .into_iter()
            .fold(self, |response, value| response.header("set-cookie", value))
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde::Serializer;
    use serde::ser::Error;

    use super::Response;

    struct Unserializable;

    impl Serialize for Unserializable {
        fn serialize<Target: Serializer>(
            &self,
            _serializer: Target,
        ) -> Result<Target::Ok, Target::Error> {
            Err(Target::Error::custom("this value cannot be serialized"))
        }
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

    #[test]
    fn serves_an_error_when_the_value_cannot_be_serialized() {
        let response = Response::json(200, &Unserializable);

        assert_eq!(response.status(), 500);
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
    fn serves_an_internal_server_error_when_the_response_is_not_a_valid_http_response() {
        let response = Response::text(9999, "unreachable status").into_http();

        assert_eq!(response.status().as_u16(), 500);
    }

    #[test]
    fn attaches_one_set_cookie_header_per_staged_cookie() {
        let response = Response::text(200, "")
            .with_cookies(vec!["first=1".to_owned(), "second=2".to_owned()])
            .into_http();

        let emitted: Vec<&str> = response
            .headers()
            .get_all("set-cookie")
            .iter()
            .map(|value| value.to_str().expect("the header is valid text"))
            .collect();

        assert_eq!(emitted, vec!["first=1", "second=2"]);
    }
}
