use std::collections::HashMap;

use crate::method::Method;

pub struct Request {
    method: Method,
    path: String,
    path_params: HashMap<String, String>,
}

impl Request {
    pub fn new(method: Method, path: String) -> Self {
        Self {
            method,
            path,
            path_params: HashMap::new(),
        }
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

    pub fn path_param(&self, name: &str) -> Option<&str> {
        self.path_params.get(name).map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Request;
    use crate::method::Method;

    #[test]
    fn exposes_method_and_path() {
        let request = Request::new(Method::Get, "/users/7".to_string());

        assert_eq!(request.method(), Method::Get);
        assert_eq!(request.path(), "/users/7");
    }

    #[test]
    fn exposes_path_parameters_once_set() {
        let mut request = Request::new(Method::Get, "/users/7".to_string());
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "7".to_string());
        request.set_path_params(path_params);

        assert_eq!(request.path_param("id"), Some("7"));
        assert_eq!(request.path_param("missing"), None);
    }
}
