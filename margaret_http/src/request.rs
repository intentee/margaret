use std::collections::HashMap;

use crate::method::Method;
use crate::request_inputs::RequestInputs;
use crate::server_params::ServerParams;
use crate::uploaded_file::UploadedFile;

pub struct Request {
    inputs: RequestInputs,
    path_params: HashMap<String, String>,
}

impl Request {
    pub(crate) fn from_inputs(inputs: RequestInputs) -> Self {
        Self {
            inputs,
            path_params: HashMap::new(),
        }
    }

    pub fn new(method: Method, path: String) -> Self {
        Self {
            inputs: RequestInputs::empty(method, path),
            path_params: HashMap::new(),
        }
    }

    pub(crate) fn with_path_params(self, path_params: HashMap<String, String>) -> Self {
        Self {
            inputs: self.inputs,
            path_params,
        }
    }

    pub fn cookie(&self, name: &str) -> Option<&str> {
        self.inputs.cookies.get(name).map(String::as_str)
    }

    pub fn file(&self, field_name: &str) -> Option<&UploadedFile> {
        self.inputs.files.get(field_name)
    }

    pub fn form(&self, key: &str) -> Option<&str> {
        self.inputs.post.get(key).map(String::as_str)
    }

    pub fn json(&self) -> Option<&serde_json::Value> {
        self.inputs.json.as_ref()
    }

    pub fn path_param(&self, name: &str) -> Option<&str> {
        self.path_params.get(name).map(String::as_str)
    }

    pub fn query(&self, key: &str) -> Option<&str> {
        self.inputs.query.get(key).map(String::as_str)
    }

    pub fn server(&self) -> &ServerParams {
        &self.inputs.server
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::net::SocketAddr;

    use http::HeaderMap;
    use tempfile::NamedTempFile;

    use super::Request;
    use crate::method::Method;
    use crate::request_inputs::RequestInputs;
    use crate::server_params::ServerParams;
    use crate::uploaded_file::UploadedFile;

    fn request_from(inputs: RequestInputs) -> Request {
        Request::from_inputs(inputs)
    }

    fn request_with_cookies(cookies: HashMap<String, String>) -> Request {
        request_from(RequestInputs {
            cookies,
            files: HashMap::new(),
            json: None,
            post: HashMap::new(),
            query: HashMap::new(),
            server: ServerParams::new(
                Method::Get,
                "/".to_string(),
                String::new(),
                SocketAddr::from(([127, 0, 0, 1], 0)),
                HeaderMap::new(),
            ),
        })
    }

    #[test]
    fn reads_a_cookie_by_name() {
        let request = request_with_cookies(HashMap::from([
            ("session".to_string(), "abc".to_string()),
            ("theme".to_string(), "dark".to_string()),
        ]));

        assert_eq!(request.cookie("session"), Some("abc"));
        assert_eq!(request.cookie("theme"), Some("dark"));
    }

    #[test]
    fn returns_none_for_a_missing_cookie() {
        assert_eq!(request_with_cookies(HashMap::new()).cookie("session"), None);
        assert_eq!(
            Request::new(Method::Get, "/".to_string()).cookie("session"),
            None
        );
    }

    #[test]
    fn exposes_query_and_form_fields() {
        let request = request_from(RequestInputs {
            cookies: HashMap::new(),
            files: HashMap::new(),
            json: None,
            post: HashMap::from([("title".to_string(), "hello".to_string())]),
            query: HashMap::from([("page".to_string(), "2".to_string())]),
            server: ServerParams::new(
                Method::Post,
                "/articles".to_string(),
                "page=2".to_string(),
                SocketAddr::from(([203, 0, 113, 7], 4000)),
                HeaderMap::new(),
            ),
        })
        .with_path_params(HashMap::from([("id".to_string(), "42".to_string())]));

        assert_eq!(request.form("title"), Some("hello"));
        assert_eq!(request.form("missing"), None);
        assert_eq!(request.query("page"), Some("2"));
        assert_eq!(request.query("missing"), None);
        assert_eq!(request.server().method(), Method::Post);
        assert_eq!(request.server().path(), "/articles");
        assert_eq!(request.server().query_string(), "page=2");
        assert_eq!(
            request.server().remote_addr(),
            SocketAddr::from(([203, 0, 113, 7], 4000))
        );
        assert_eq!(request.path_param("id"), Some("42"));
        assert_eq!(request.path_param("missing"), None);
    }

    #[test]
    fn exposes_uploaded_files() {
        let avatar = NamedTempFile::new()
            .expect("a temporary file")
            .into_temp_path();
        let attachment = NamedTempFile::new()
            .expect("a temporary file")
            .into_temp_path();
        let request = request_from(RequestInputs {
            cookies: HashMap::new(),
            files: HashMap::from([
                (
                    "avatar".to_string(),
                    UploadedFile::new(
                        "avatar".to_string(),
                        "face.png".to_string(),
                        "image/png".to_string(),
                        3,
                        avatar,
                    ),
                ),
                (
                    "attachment".to_string(),
                    UploadedFile::new(
                        "attachment".to_string(),
                        "notes.txt".to_string(),
                        "text/plain".to_string(),
                        5,
                        attachment,
                    ),
                ),
            ]),
            json: None,
            post: HashMap::new(),
            query: HashMap::new(),
            server: ServerParams::new(
                Method::Post,
                "/upload".to_string(),
                String::new(),
                SocketAddr::from(([127, 0, 0, 1], 0)),
                HeaderMap::new(),
            ),
        });

        let avatar = request.file("avatar").expect("the avatar file is present");
        assert_eq!(avatar.file_name(), "face.png");
        assert_eq!(avatar.content_type(), "image/png");
        assert_eq!(avatar.size(), 3);
        assert!(request.file("missing").is_none());
    }

    #[test]
    fn exposes_the_json_body() {
        let request = request_from(RequestInputs {
            cookies: HashMap::new(),
            files: HashMap::new(),
            json: Some(serde_json::json!({ "title": "hello" })),
            post: HashMap::new(),
            query: HashMap::new(),
            server: ServerParams::new(
                Method::Post,
                "/articles".to_string(),
                String::new(),
                SocketAddr::from(([127, 0, 0, 1], 0)),
                HeaderMap::new(),
            ),
        });

        assert!(request.json() == Some(&serde_json::json!({ "title": "hello" })));
    }
}
