use margaret_http::bearer_challenge::BearerChallenge;
use margaret_jwt_verification::jwt_rejection::JwtRejection;

#[derive(Debug)]
pub enum OidcTokenRejection {
    MalformedAuthorization,
    Token(JwtRejection),
}

impl OidcTokenRejection {
    #[must_use]
    pub fn challenge(&self) -> BearerChallenge {
        match self {
            Self::MalformedAuthorization => BearerChallenge::InvalidRequest,
            Self::Token(_) => BearerChallenge::InvalidToken,
        }
    }
}
