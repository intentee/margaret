use headers::HeaderMapExt;
use http::HeaderMap;
use reqwest::RequestBuilder;

use margaret_oauth_vocabulary::client_secret_basic::client_secret_basic;

pub enum ClientCredentials {
    Absent,
    Asserted {
        client_id: &'static str,
    },
    Assertion(String),
    Basic {
        client_id: &'static str,
        secret: &'static str,
    },
    Bearer(String),
}

impl ClientCredentials {
    /// # Panics
    ///
    /// Panics when the basic client secret is malformed.
    pub fn presented_on(&self, request: RequestBuilder) -> RequestBuilder {
        match self {
            Self::Absent | Self::Asserted { .. } | Self::Assertion(_) => request,
            Self::Basic { client_id, secret } => {
                let mut headers = HeaderMap::new();

                headers.typed_insert(client_secret_basic(
                    client_id,
                    &secret.parse().expect("the client secret is not empty"),
                ));

                request.headers(headers)
            }
            Self::Bearer(token) => request.bearer_auth(token),
        }
    }
}
