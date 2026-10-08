use crate::declared_client_credentials::DeclaredClientCredentials;
use crate::declared_client_keys::DeclaredClientKeys;

#[derive(Debug)]
pub struct DeclaredConfidentialClient {
    pub client_credentials: DeclaredClientCredentials,
    pub introspection: bool,
    pub keys: DeclaredClientKeys,
}
