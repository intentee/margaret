use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_jwt_verification::jwt_rejection::JwtRejection;

#[derive(Debug)]
pub enum SubjectTokenRefusal {
    ExchangeRefused,
    Rejected(JwtRejection),
    TokenTypeMismatch,
    UntrustedIssuer { issuer: String },
}

impl Display for SubjectTokenRefusal {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::ExchangeRefused => formatter.write_str("the exchanger refused the subject token"),
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
