use bytes::Bytes;
use http::StatusCode;
use http_body_util::Full;
use maud::Markup;
use serde::Serialize;

use crate::header::Header;
use crate::security_headers::apply_security_headers;

fn internal_server_error() -> http::Response<Full<Bytes>> {
    let mut response = http::Response::new(Full::new(Bytes::from_static(b"Internal Server Error")));

    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    apply_security_headers(response.headers_mut());

    response
}

pub struct Response {
    body: Bytes,
    headers: Vec<Header>,
    status: u16,
}

impl Response {
    pub fn bytes(status: u16, content_type: impl Into<String>, body: impl Into<Bytes>) -> Self {
        Self::new(status, body.into()).header("content-type", content_type)
    }

    #[must_use]
    pub fn forbidden() -> Self {
        Self::text(403, "Forbidden")
    }

    pub(crate) fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push(Header {
            name: name.into(),
            value: value.into(),
        });

        self
    }

    #[must_use]
    pub fn immutable_asset(self) -> Self {
        self.header("cache-control", "public, max-age=31536000, immutable")
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

    fn new(status: u16, body: Bytes) -> Self {
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
    pub fn revalidating_asset(self) -> Self {
        self.header("cache-control", "no-cache")
    }

    #[must_use]
    pub fn static_bytes(status: u16, content_type: &'static str, body: &'static [u8]) -> Self {
        Self::new(status, Bytes::from_static(body)).header("content-type", content_type)
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn text(status: u16, body: impl Into<String>) -> Self {
        Self::new(status, Bytes::from(body.into()))
    }

    #[must_use]
    pub fn unauthorized() -> Self {
        Self::text(401, "Unauthorized")
    }

    #[must_use]
    pub fn web_socket_upgrade(accept: String) -> Self {
        Self::text(101, "")
            .header("connection", "Upgrade")
            .header("sec-websocket-accept", accept)
            .header("upgrade", "websocket")
    }

    pub(crate) fn into_http(self) -> http::Response<Full<Bytes>> {
        let status = self.status;
        let mut builder = http::Response::builder().status(status);

        for header in self.headers {
            builder = builder.header(header.name, header.value);
        }

        match builder.body(Full::new(self.body)) {
            Ok(mut response) => {
                apply_security_headers(response.headers_mut());

                response
            }
            Err(error) => {
                eprintln!(
                    "margaret_http: the responder produced a response that is not a valid HTTP response (status `{status}`): {error}"
                );

                internal_server_error()
            }
        }
    }
}

impl From<Markup> for Response {
    fn from(markup: Markup) -> Self {
        Self::html(200, markup.into_string())
    }
}

#[cfg(test)]
mod tests {
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
    fn applies_mandatory_security_headers_to_every_response() {
        let response = Response::text(200, "ok").into_http();

        assert_eq!(response.headers()["x-content-type-options"], "nosniff");
        assert_eq!(response.headers()["x-frame-options"], "DENY");
        assert_eq!(response.headers()["referrer-policy"], "no-referrer");
        assert_eq!(
            response.headers()["strict-transport-security"],
            "max-age=63072000; includeSubDomains"
        );
        assert_eq!(
            response.headers()["cross-origin-opener-policy"],
            "same-origin"
        );
        assert_eq!(
            response.headers()["cross-origin-resource-policy"],
            "same-origin"
        );
        assert!(response.headers().contains_key("content-security-policy"));
        assert!(response.headers().contains_key("permissions-policy"));
        assert_eq!(response.headers()["cache-control"], "no-store");
    }

    #[test]
    fn application_code_cannot_override_mandatory_security_headers() {
        let response = Response::text(200, "ok")
            .header("content-security-policy", "default-src *")
            .header("x-content-type-options", "unsafe")
            .into_http();

        assert_eq!(
            response.headers()["content-security-policy"],
            "default-src 'none'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'"
        );
        assert_eq!(response.headers()["x-content-type-options"], "nosniff");
    }

    #[test]
    fn applies_the_immutable_asset_cache_policy() {
        let response = Response::text(200, "ok").immutable_asset().into_http();

        assert_eq!(
            response.headers()["cache-control"],
            "public, max-age=31536000, immutable"
        );
    }

    #[test]
    fn applies_the_revalidating_asset_cache_policy() {
        let response = Response::text(200, "ok").revalidating_asset().into_http();

        assert_eq!(response.headers()["cache-control"], "no-cache");
    }

    #[test]
    fn builds_a_web_socket_upgrade_response() {
        let response = Response::web_socket_upgrade("derived-accept".to_string()).into_http();

        assert_eq!(response.status().as_u16(), 101);
        assert_eq!(response.headers()["connection"], "Upgrade");
        assert_eq!(response.headers()["sec-websocket-accept"], "derived-accept");
        assert_eq!(response.headers()["upgrade"], "websocket");
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
