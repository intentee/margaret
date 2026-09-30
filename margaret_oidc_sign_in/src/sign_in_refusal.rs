use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use oauth2::basic::BasicErrorResponse;

use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_registered_claims::audience_claim::AudienceClaim;

#[derive(Debug)]
pub enum SignInRefusal {
    AudienceNotExclusive {
        found: AudienceClaim,
    },
    AuthorizationDenied {
        description: Option<String>,
        error: String,
    },
    AuthorizedPartyMismatch {
        found: String,
    },
    CodeMissing,
    IdTokenRejected(JwtRejection),
    IssuerMismatch {
        found: String,
    },
    IssuerMissing,
    NonceMismatch,
    StateMismatch,
    TokenRefused(BasicErrorResponse),
    TransactionMissing,
    TransactionRejected(JwtRejection),
}

impl Display for SignInRefusal {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::AudienceNotExclusive { found } => write!(
                formatter,
                "the id token is also meant for audiences other than this client: {found}"
            ),
            Self::AuthorizationDenied {
                description: Some(description),
                error,
            } => write!(
                formatter,
                "the authorization server denied the sign-in with '{error}': {description}"
            ),
            Self::AuthorizationDenied {
                description: None,
                error,
            } => write!(
                formatter,
                "the authorization server denied the sign-in with '{error}'"
            ),
            Self::AuthorizedPartyMismatch { found } => write!(
                formatter,
                "the id token was issued to the authorized party '{found}' instead of this client"
            ),
            Self::CodeMissing => {
                formatter.write_str("the authorization response carries no authorization code")
            }
            Self::IdTokenRejected(rejection) => {
                write!(formatter, "the id token is rejected: {rejection}")
            }
            Self::IssuerMismatch { found } => write!(
                formatter,
                "the authorization response was issued by '{found}' instead of the trusted issuer"
            ),
            Self::IssuerMissing => formatter.write_str(
                "the authorization response does not name its issuer although the issuer advertises it",
            ),
            Self::NonceMismatch => {
                formatter.write_str("the id token does not carry the nonce of the sign-in")
            }
            Self::StateMismatch => formatter
                .write_str("the authorization response does not carry the state of the sign-in"),
            Self::TokenRefused(refusal) => write!(
                formatter,
                "the authorization server refused to exchange the authorization code: {refusal}"
            ),
            Self::TransactionMissing => {
                formatter.write_str("the browser presents no sign-in transaction")
            }
            Self::TransactionRejected(rejection) => write!(
                formatter,
                "the sign-in transaction is rejected: {rejection}"
            ),
        }
    }
}
