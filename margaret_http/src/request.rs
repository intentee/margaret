use std::collections::HashMap;

use bytes::Bytes;
use cookie::Cookie;
use http::HeaderMap;

use crate::method::Method;
use crate::request_error::RequestError;

#[derive(Clone)]
pub struct Request {
    body: Bytes,
    headers: HeaderMap,
    method: Method,
    path: String,
    path_params: HashMap<String, String>,
}

impl Request {
    pub fn new(method: Method, path: String) -> Self {
        Self {
            body: Bytes::new(),
            headers: HeaderMap::new(),
            method,
            path,
            path_params: HashMap::new(),
        }
    }

    pub(crate) fn set_body(&mut self, body: Bytes) {
        self.body = body;
    }

    pub(crate) fn set_headers(&mut self, headers: HeaderMap) {
        self.headers = headers;
    }

    pub(crate) fn set_path_params(&mut self, path_params: HashMap<String, String>) {
        self.path_params = path_params;
    }

    pub(crate) fn clear_path_params(&mut self) {
        self.path_params.clear();
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn header(&self, name: &str) -> Result<Option<&str>, RequestError> {
        match self.headers.get(name) {
            Some(value) => Ok(Some(value.to_str()?)),
            None => Ok(None),
        }
    }

    pub fn path_param(&self, name: &str) -> Option<&str> {
        self.path_params.get(name).map(String::as_str)
    }

    pub fn cookie(&self, name: &str) -> Result<Option<String>, RequestError> {
        let Some(raw) = self.header("cookie")? else {
            return Ok(None);
        };

        for parsed in Cookie::split_parse(raw) {
            let cookie = parsed?;

            if cookie.name() == name {
                return Ok(Some(cookie.value().to_string()));
            }
        }

        Ok(None)
    }

    pub fn form(&self, key: &str) -> Option<String> {
        form_urlencoded::parse(&self.body).find_map(|(field, value)| {
            if &*field == key {
                Some(value.into_owned())
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use http::HeaderMap;
    use http::HeaderName;
    use http::HeaderValue;

    use super::Request;
    use crate::method::Method;

    fn request_with_header(name: &str, value: &str) -> Request {
        let mut request = Request::new(Method::Get, "/".to_string());
        let mut headers = HeaderMap::new();

        headers.insert(
            HeaderName::from_bytes(name.as_bytes()).expect("a valid header name"),
            HeaderValue::from_str(value).expect("a valid header value"),
        );
        request.set_headers(headers);

        request
    }

    #[test]
    fn reads_a_cookie_by_name() {
        let request = request_with_header("cookie", "session=abc; theme=dark");

        assert_eq!(
            request.cookie("session").expect("the cookie header parses"),
            Some("abc".to_string())
        );
        assert_eq!(
            request.cookie("theme").expect("the cookie header parses"),
            Some("dark".to_string())
        );
    }

    #[test]
    fn returns_none_for_a_missing_cookie() {
        assert_eq!(
            request_with_header("cookie", "theme=dark")
                .cookie("session")
                .expect("the cookie header parses"),
            None
        );
        assert_eq!(
            Request::new(Method::Get, "/".to_string())
                .cookie("session")
                .expect("an absent cookie header yields no cookie"),
            None
        );
    }

    #[test]
    fn rejects_a_header_value_that_is_not_visible_ascii() {
        let mut request = Request::new(Method::Get, "/".to_string());
        let mut headers = HeaderMap::new();

        headers.insert(
            HeaderName::from_static("cookie"),
            HeaderValue::from_bytes(&[0xC0, 0xC1]).expect("an opaque header value"),
        );
        request.set_headers(headers);

        let message = request
            .header("cookie")
            .expect_err("an opaque header value is rejected")
            .to_string();

        assert!(message.contains("not visible ASCII text"));
        assert!(request.cookie("session").is_err());
    }

    #[test]
    fn rejects_a_malformed_cookie_header() {
        let message = request_with_header("cookie", "=nameless")
            .cookie("session")
            .expect_err("a malformed cookie is rejected")
            .to_string();

        assert!(message.contains("could not be parsed"));
    }

    #[test]
    fn reads_a_form_field_from_the_body() {
        let mut request = Request::new(Method::Post, "/login".to_string());

        request.set_body(Bytes::from_static(b"username=margaret&password=secret"));

        assert_eq!(request.form("username"), Some("margaret".to_string()));
        assert_eq!(request.form("password"), Some("secret".to_string()));
        assert_eq!(request.form("absent"), None);
    }
}
