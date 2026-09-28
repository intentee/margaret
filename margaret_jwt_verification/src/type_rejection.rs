use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification::header_type::HeaderType;

#[derive(Debug)]
pub enum TypeRejection {
    Mismatch {
        expected: JwtType,
        found: HeaderType,
    },
    Missing {
        expected: JwtType,
    },
}

impl Display for TypeRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Mismatch { expected, found } => write!(
                formatter,
                "the token declares the type '{found}' where '{expected}' is expected"
            ),
            Self::Missing { expected } => write!(
                formatter,
                "the token does not declare its type, and '{expected}' is expected"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_jose_parameters::jwt_type::JwtType;
    use margaret_jws_verification::header_type::HeaderType;

    use super::TypeRejection;

    #[test]
    fn describes_every_rejection() {
        assert_eq!(
            TypeRejection::Mismatch {
                expected: JwtType::AccessToken,
                found: HeaderType::Unsupported("dpop+jwt".to_string()),
            }
            .to_string(),
            "the token declares the type 'dpop+jwt' where 'at+jwt' is expected"
        );
        assert_eq!(
            TypeRejection::Missing {
                expected: JwtType::Jwt,
            }
            .to_string(),
            "the token does not declare its type, and 'JWT' is expected"
        );
    }
}
