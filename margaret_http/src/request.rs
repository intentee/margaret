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

    #[must_use]
    pub fn new(method: Method, path: String) -> Self {
        Self {
            inputs: RequestInputs::synthetic(method, path),
            path_params: HashMap::new(),
            peer_identity: Arc::new(PeerIdentity::Anonymous),
        }
    }

    pub fn path_param(&self, name: &str) -> Option<&str> {
        self.path_params.get(name).map(String::as_str)
    }

    #[must_use]
    pub fn peer_identity(&self) -> &PeerIdentity {
        &self.peer_identity
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
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use http::Method;

    use super::Request;
    use crate::request_body_inputs::RequestBodyInputs;
    use crate::request_inputs::RequestInputs;
    use crate::server_params::ServerParams;

    #[test]
    fn exposes_its_inputs_and_path_parameters() {
        let request = Request::from_inputs(RequestInputs {
            body: RequestBodyInputs::UrlEncoded(HashMap::from([(
                "title".to_string(),
                "hello".to_string(),
            )])),
            cookies: HashMap::new(),
            query: HashMap::from([("page".to_string(), "2".to_string())]),
            server: ServerParams::synthetic(Method::POST, "/articles".to_string()),
        })
        .with_path_params(HashMap::from([("id".to_string(), "42".to_string())]));

        assert_eq!(
            request.inputs.body.form().get("title").map(String::as_str),
            Some("hello")
        );
        assert_eq!(
            request.inputs.query.get("page").map(String::as_str),
            Some("2")
        );
        assert_eq!(request.inputs.server.method(), "POST");
        assert_eq!(request.inputs.server.path(), "/articles");
        assert_eq!(request.path_param("id"), Some("42"));
        assert_eq!(request.path_param("missing"), None);
    }
}
