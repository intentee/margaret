#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClientAuthenticationMethod {
    ClientSecretBasic,
    None,
    PrivateKeyJwt,
}

impl ClientAuthenticationMethod {
    pub const ALL: [Self; 3] = [Self::ClientSecretBasic, Self::None, Self::PrivateKeyJwt];

    #[must_use]
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::ClientSecretBasic => "client_secret_basic",
            Self::None => "none",
            Self::PrivateKeyJwt => "private_key_jwt",
        }
    }
}
