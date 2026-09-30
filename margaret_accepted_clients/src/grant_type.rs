#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GrantType {
    AuthorizationCode,
    ClientCredentials,
    RefreshToken,
    TokenExchange,
}

impl GrantType {
    #[must_use]
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::AuthorizationCode => "authorization_code",
            Self::ClientCredentials => "client_credentials",
            Self::RefreshToken => "refresh_token",
            Self::TokenExchange => "urn:ietf:params:oauth:grant-type:token-exchange",
        }
    }
}
