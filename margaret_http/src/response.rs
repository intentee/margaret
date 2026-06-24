use bytes::Bytes;
use http_body_util::Full;

pub struct Response {
    body: String,
    headers: Vec<(String, String)>,
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

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));

        self
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub(crate) fn into_http(self) -> http::Response<Full<Bytes>> {
        let mut builder = http::Response::builder().status(self.status);

        for (name, value) in self.headers {
            builder = builder.header(name, value);
        }

        builder
            .body(Full::new(Bytes::from(self.body)))
            .expect("a well-formed response is built")
    }
}

#[cfg(test)]
mod tests {
    use super::Response;

    #[test]
    fn builds_an_http_response_with_status_and_headers() {
        let response = Response::text(201, "created")
            .header("x-marker", "on")
            .into_http();

        assert_eq!(response.status().as_u16(), 201);
        assert!(response.headers().contains_key("x-marker"));
    }
}
