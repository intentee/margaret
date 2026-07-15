use std::collections::HashMap;
use std::sync::Arc;

use http::Method;

use margaret_peer_identity::peer_identity::PeerIdentity;

use crate::request_inputs::RequestInputs;

pub struct Request {
    pub inputs: RequestInputs,
    path_params: HashMap<String, String>,
    peer_identity: Arc<PeerIdentity>,
}

impl Request {
    pub(crate) fn from_inputs(inputs: RequestInputs) -> Self {
        Self {
            inputs,
            path_params: HashMap::new(),
            peer_identity: Arc::new(PeerIdentity::Anonymous),
        }
    }

    pub fn new(method: Method, path: String) -> Self {
        Self {
            inputs: RequestInputs::empty(method, path),
            path_params: HashMap::new(),
            peer_identity: Arc::new(PeerIdentity::Anonymous),
        }
    }

    pub(crate) fn with_path_params(self, path_params: HashMap<String, String>) -> Self {
        Self {
            inputs: self.inputs,
            path_params,
            peer_identity: self.peer_identity,
        }
    }

    pub(crate) fn with_peer_identity(self, peer_identity: Arc<PeerIdentity>) -> Self {
        Self {
            inputs: self.inputs,
            path_params: self.path_params,
            peer_identity,
        }
    }

    pub fn path_param(&self, name: &str) -> Option<&str> {
        self.path_params.get(name).map(String::as_str)
    }

    pub fn peer_identity(&self) -> &PeerIdentity {
        &self.peer_identity
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
