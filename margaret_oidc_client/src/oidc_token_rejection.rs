use std::sync::Arc;

use margaret_http::bearer_challenge::BearerChallenge;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;

#[derive(Debug)]
pub enum OidcTokenRejection {
    MalformedAuthorization,
    Token(JwtRejection),
    UnparseableToken(Arc<JwsRejection>),
}

impl OidcTokenRejection {
    #[must_use]
    pub fn challenge(&self) -> BearerChallenge {
        match self {
            Self::MalformedAuthorization => BearerChallenge::InvalidRequest,
            Self::Token(_) | Self::UnparseableToken(_) => BearerChallenge::InvalidToken,
        }
    }
}
