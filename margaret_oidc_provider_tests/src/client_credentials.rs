use headers::HeaderMapExt;
use http::HeaderMap;
use reqwest::RequestBuilder;

use margaret_oauth_vocabulary::client_secret_basic::ClientSecretBasic;

pub enum ClientCredentials {
    Absent,
    Basic {
        client_id: &'static str,
        secret: &'static str,
    },
    Bearer(String),
}

impl ClientCredentials {
    /// # Panics
    ///
    /// Panics when the basic client identifier or secret is malformed.
    pub fn presented_on(&self, request: RequestBuilder) -> RequestBuilder {
        match self {
            Self::Absent => request,
            Self::Basic { client_id, secret } => {
                let mut headers = HeaderMap::new();

                headers.typed_insert(ClientSecretBasic::authorization(
                    &client_id.parse().expect("the client identifier is visible"),
                    &secret.parse().expect("the client secret is not empty"),
                ));

                request.headers(headers)
            }
            Self::Bearer(token) => request.bearer_auth(token),
        }
    }
}
