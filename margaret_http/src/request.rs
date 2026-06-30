use std::collections::HashMap;

use cookie::Cookie;

use crate::method::Method;

pub struct Request {
    body: String,
    headers: HashMap<String, String>,
    method: Method,
    path: String,
    path_params: HashMap<String, String>,
}

impl Request {
    pub fn new(method: Method, path: String) -> Self {
        Self {
            body: String::new(),
            headers: HashMap::new(),
            method,
            path,
            path_params: HashMap::new(),
        }
    }

    pub(crate) fn set_body(&mut self, body: String) {
        self.body = body;
    }

    pub(crate) fn set_headers(&mut self, headers: HashMap<String, String>) {
        self.headers = headers;
    }

    pub(crate) fn set_path_params(&mut self, path_params: HashMap<String, String>) {
        self.path_params = path_params;
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).map(String::as_str)
    }

    pub fn path_param(&self, name: &str) -> Option<&str> {
        self.path_params.get(name).map(String::as_str)
    }

    pub fn cookie(&self, name: &str) -> Option<String> {
        let raw = self.header("cookie")?;

        Cookie::split_parse(raw)
            .filter_map(Result::ok)
            .find(|cookie| cookie.name() == name)
            .map(|cookie| cookie.value().to_string())
    }

    pub fn form(&self, key: &str) -> Option<String> {
        form_urlencoded::parse(self.body.as_bytes()).find_map(|(field, value)| {
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
    use std::collections::HashMap;

    use super::Request;
    use crate::method::Method;

    fn request_with_header(name: &str, value: &str) -> Request {
        let mut request = Request::new(Method::Get, "/".to_string());
        let mut headers = HashMap::new();

        headers.insert(name.to_string(), value.to_string());
        request.set_headers(headers);

        request
    }

    #[test]
    fn reads_a_cookie_by_name() {
        let request = request_with_header("cookie", "session=abc; theme=dark");

        assert_eq!(request.cookie("session"), Some("abc".to_string()));
        assert_eq!(request.cookie("theme"), Some("dark".to_string()));
    }

    #[test]
    fn returns_none_for_a_missing_cookie() {
        assert_eq!(
            request_with_header("cookie", "theme=dark").cookie("session"),
            None
        );
        assert_eq!(
            Request::new(Method::Get, "/".to_string()).cookie("session"),
            None
        );
    }

    #[test]
    fn reads_a_form_field_from_the_body() {
        let mut request = Request::new(Method::Post, "/login".to_string());

        request.set_body("username=margaret&password=secret".to_string());

        assert_eq!(request.form("username"), Some("margaret".to_string()));
        assert_eq!(request.form("password"), Some("secret".to_string()));
        assert_eq!(request.form("absent"), None);
    }
}
