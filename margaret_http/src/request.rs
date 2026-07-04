use std::collections::HashMap;

use http::Method;

use crate::request_inputs::RequestInputs;

pub struct Request {
    pub inputs: RequestInputs,
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

    pub fn path_param(&self, name: &str) -> Option<&str> {
        self.path_params.get(name).map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::net::SocketAddr;

    use http::HeaderMap;
    use http::Method;

    use super::Request;
    use crate::request_inputs::RequestInputs;
    use crate::server_params::ServerParams;

    #[test]
    fn exposes_its_inputs_and_path_parameters() {
        let request = Request::from_inputs(RequestInputs {
            cookies: HashMap::new(),
            files: HashMap::new(),
            json: None,
            form: HashMap::from([("title".to_string(), "hello".to_string())]),
            query: HashMap::from([("page".to_string(), "2".to_string())]),
            server: ServerParams::new(
                Method::POST,
                "/articles".to_string(),
                "page=2".to_string(),
                SocketAddr::from(([203, 0, 113, 7], 4000)),
                HeaderMap::new(),
            ),
        })
        .with_path_params(HashMap::from([("id".to_string(), "42".to_string())]));

        assert_eq!(
            request.inputs.form.get("title").map(String::as_str),
            Some("hello")
        );
        assert_eq!(
            request.inputs.query.get("page").map(String::as_str),
            Some("2")
        );
        assert_eq!(request.inputs.server.method(), "POST");
        assert_eq!(request.inputs.server.path(), "/articles");
        assert_eq!(request.inputs.server.query_string(), "page=2");
        assert_eq!(
            request.inputs.server.remote_addr(),
            SocketAddr::from(([203, 0, 113, 7], 4000))
        );
        assert_eq!(request.path_param("id"), Some("42"));
        assert_eq!(request.path_param("missing"), None);
    }
}
