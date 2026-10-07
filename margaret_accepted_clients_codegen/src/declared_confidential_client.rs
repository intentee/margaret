use margaret_https_url::https_url::HttpsUrl;

use crate::declared_client_credentials::DeclaredClientCredentials;

#[derive(Debug)]
pub struct DeclaredConfidentialClient {
    pub client_credentials: DeclaredClientCredentials,
    pub introspection: bool,
    pub jwks_uri: HttpsUrl,
}
