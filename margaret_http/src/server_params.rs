use std::net::SocketAddr;

use http::HeaderMap;
use http::Method;

use crate::request_error::RequestError;

pub struct ServerParams {
    headers: HeaderMap,
    method: Method,
    path: String,
    query_string: String,
    remote_addr: SocketAddr,
}

impl ServerParams {
    pub(crate) fn new(
        method: Method,
        path: String,
        query_string: String,
        remote_addr: SocketAddr,
        headers: HeaderMap,
    ) -> Self {
        Self {
            headers,
            method,
            path,
            query_string,
            remote_addr,
        }
    }

    pub fn header(&self, name: &str) -> Result<Option<&str>, RequestError> {
        match self.headers.get(name) {
            Some(value) => Ok(Some(value.to_str()?)),
            None => Ok(None),
        }
    }

    pub fn method(&self) -> &str {
        self.method.as_str()
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn query_string(&self) -> &str {
        &self.query_string
    }

    pub fn remote_addr(&self) -> SocketAddr {
        self.remote_addr
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use http::HeaderMap;
    use http::HeaderName;
    use http::HeaderValue;

    use http::Method;

    use super::ServerParams;

    fn params_with_header(name: &str, value: &[u8]) -> ServerParams {
        let mut headers = HeaderMap::new();

        headers.insert(
            HeaderName::from_bytes(name.as_bytes()).expect("a valid header name"),
            HeaderValue::from_bytes(value).expect("a valid header value"),
        );

        ServerParams::new(
            Method::GET,
            "/".to_string(),
            String::new(),
            SocketAddr::from(([127, 0, 0, 1], 0)),
            headers,
        )
    }

    #[test]
    fn reads_a_header_value() {
        assert_eq!(
            params_with_header("x-token", b"secret")
                .header("x-token")
                .expect("the header value is ascii"),
            Some("secret")
        );
    }

    #[test]
    fn returns_none_for_a_missing_header() {
        assert_eq!(
            params_with_header("x-token", b"secret")
                .header("absent")
                .expect("a missing header yields no value"),
            None
        );
    }

    #[test]
    fn rejects_a_non_ascii_header_value() {
        let message = params_with_header("x-token", &[0xC0, 0xC1])
            .header("x-token")
            .err()
            .unwrap()
            .to_string();

        assert!(message.contains("not visible ASCII text"));
    }
}
