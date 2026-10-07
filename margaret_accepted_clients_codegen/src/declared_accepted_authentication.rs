use crate::declared_confidential_client::DeclaredConfidentialClient;

#[derive(Debug)]
pub enum DeclaredAcceptedAuthentication {
    PrivateKeyJwt(DeclaredConfidentialClient),
    Public,
}
