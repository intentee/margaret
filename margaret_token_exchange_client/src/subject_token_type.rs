#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubjectTokenType {
    AccessToken,
    IdToken,
    Jwt,
}

impl SubjectTokenType {
    pub const ALL: [Self; 3] = [Self::AccessToken, Self::IdToken, Self::Jwt];

    #[must_use]
    pub fn urn(self) -> &'static str {
        match self {
            Self::AccessToken => "urn:ietf:params:oauth:token-type:access_token",
            Self::IdToken => "urn:ietf:params:oauth:token-type:id_token",
            Self::Jwt => "urn:ietf:params:oauth:token-type:jwt",
        }
    }
}
