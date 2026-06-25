use std::collections::HashMap;

use crate::method::Method;

pub struct Request {
    headers: HashMap<String, String>,
    method: Method,
    path: String,
    path_params: HashMap<String, String>,
}

impl Request {
    pub fn new(method: Method, path: String) -> Self {
        Self {
            headers: HashMap::new(),
            method,
            path,
            path_params: HashMap::new(),
        }
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
}
