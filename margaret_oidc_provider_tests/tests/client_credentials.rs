use reqwest::RequestBuilder;

pub enum ClientCredentials {
    Absent,
    Basic {
        client_id: &'static str,
        secret: &'static str,
    },
    Bearer(String),
}

impl ClientCredentials {
    pub fn presented_on(&self, request: RequestBuilder) -> RequestBuilder {
        match self {
            Self::Absent => request,
            Self::Basic { client_id, secret } => request.basic_auth(client_id, Some(secret)),
            Self::Bearer(token) => request.bearer_auth(token),
        }
    }
}
