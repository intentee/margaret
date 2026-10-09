use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_registered_claims::audience_claim::AudienceClaim;

#[derive(Debug)]
pub enum SubjectTokenRefusal {
    Ambiguous {
        audience: AudienceClaim,
        issuer: String,
    },
    ExchangeRefused,
    Misaddressed {
        audience: AudienceClaim,
        issuer: String,
    },
    Rejected(JwtRejection),
    TokenTypeMismatch,
    UntrustedIssuer {
        issuer: String,
    },
}

impl Display for SubjectTokenRefusal {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Ambiguous { audience, issuer } => write!(
                formatter,
                "the subject token issued by '{issuer}' for {audience} is addressed to more than one exchanger"
            ),
            Self::ExchangeRefused => formatter.write_str("the exchanger refused the subject token"),
            Self::Misaddressed { audience, issuer } => write!(
                formatter,
                "the subject token issued by '{issuer}' for {audience} is addressed to no exchanger"
            ),
            Self::Rejected(rejection) => {
                write!(formatter, "the subject token is rejected: {rejection}")
            }
            Self::TokenTypeMismatch => formatter
                .write_str("the subject token type does not match the profile of its exchanger"),
            Self::UntrustedIssuer { issuer } => write!(
                formatter,
                "the subject token is issued by '{issuer}', which no exchanger trusts"
            ),
        }
    }
}
