use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_jws_verification::jws_rejection::JwsRejection;

use crate::claims_rejection::ClaimsRejection;
use crate::type_rejection::TypeRejection;

#[derive(Debug)]
pub enum JwtRejection {
    Claims(ClaimsRejection),
    Jws(JwsRejection),
    Type(TypeRejection),
}

impl Display for JwtRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Claims(rejection) => rejection.fmt(formatter),
            Self::Jws(rejection) => rejection.fmt(formatter),
            Self::Type(rejection) => rejection.fmt(formatter),
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_jose_parameters::jwt_type::JwtType;
    use margaret_jws_verification::jws_rejection::JwsRejection;
    use margaret_registered_claims::numeric_date::NumericDate;

    use super::JwtRejection;
    use crate::claims_rejection::ClaimsRejection;
    use crate::type_rejection::TypeRejection;

    #[test]
    fn describes_a_claims_rejection_as_the_claims_rejection() {
        let rejection = ClaimsRejection::Expired {
            exp: NumericDate::new(100),
            now: NumericDate::new(150),
        };
        let expected = rejection.to_string();

        assert_eq!(JwtRejection::Claims(rejection).to_string(), expected);
    }

    #[test]
    fn describes_a_type_rejection_as_the_type_rejection() {
        let rejection = TypeRejection::Missing {
            expected: JwtType::Jwt,
        };
        let expected = rejection.to_string();

        assert_eq!(JwtRejection::Type(rejection).to_string(), expected);
    }

    #[test]
    fn describes_a_jws_rejection_as_the_jws_rejection() {
        assert_eq!(
            JwtRejection::Jws(JwsRejection::MissingKeyId).to_string(),
            JwsRejection::MissingKeyId.to_string()
        );
    }
}
