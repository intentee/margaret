use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use chrono::DateTime;
use chrono::Utc;

#[derive(Debug)]
pub enum IntrospectionRejection {
    AudienceMismatch {
        found: Vec<String>,
    },
    AudienceMissing,
    Expired {
        exp: DateTime<Utc>,
        now: DateTime<Utc>,
    },
    Inactive,
    IssuerMismatch {
        found: String,
    },
    NotYetValid {
        nbf: DateTime<Utc>,
        now: DateTime<Utc>,
    },
    UnexpectedClaims {
        source: serde_json::Error,
    },
}

impl Display for IntrospectionRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::AudienceMismatch { found } => write!(
                formatter,
                "the introspected token's audience {found:?} does not include ours"
            ),
            Self::AudienceMissing => {
                formatter.write_str("the introspected token names no audience")
            }
            Self::Expired { exp, now } => write!(
                formatter,
                "the introspected token expired at {exp}, it is {now}"
            ),
            Self::Inactive => formatter.write_str("the introspected token is not active"),
            Self::IssuerMismatch { found } => write!(
                formatter,
                "the introspected token was issued by '{found}' instead of its authorization server"
            ),
            Self::NotYetValid { nbf, now } => write!(
                formatter,
                "the introspected token is not valid before {nbf}, it is {now}"
            ),
            Self::UnexpectedClaims { source } => write!(
                formatter,
                "the introspected token does not carry the expected claims: {source}"
            ),
        }
    }
}
