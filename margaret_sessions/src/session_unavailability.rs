use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use http::StatusCode;

use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;
use margaret_jwt_verification::jwt_rejection::JwtRejection;

#[derive(Debug)]
pub enum SessionUnavailability {
    KeysAwaited,
    MalformedRefreshAnswer { source: serde_json::Error },
    OversizedRefreshAnswer { max_bytes: usize },
    RefreshExchange(IssuerExchangeError),
    RefreshStatus { status: StatusCode },
    UnverifiableRefreshedToken(JwtRejection),
}

impl Display for SessionUnavailability {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::KeysAwaited => {
                formatter.write_str("the signing keys of the session issuer are not available yet")
            }
            Self::MalformedRefreshAnswer { source } => write!(
                formatter,
                "the session issuer answered the refresh with a malformed body: {source}"
            ),
            Self::OversizedRefreshAnswer { max_bytes } => write!(
                formatter,
                "the session issuer answered the refresh with more than {max_bytes} bytes"
            ),
            Self::RefreshExchange(failure) => {
                write!(formatter, "the session issuer cannot be reached: {failure}")
            }
            Self::RefreshStatus { status } => write!(
                formatter,
                "the session issuer answered the refresh with status {status}"
            ),
            Self::UnverifiableRefreshedToken(rejection) => write!(
                formatter,
                "the session issuer refreshed the session with an access token that does not verify: {rejection}"
            ),
        }
    }
}

impl From<SessionUnavailability> for ResponseContinuation {
    fn from(unavailability: SessionUnavailability) -> Self {
        Self::from(Response::text(503, unavailability.to_string()))
    }
}
