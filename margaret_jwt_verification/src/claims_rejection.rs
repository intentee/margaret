use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_registered_claims::numeric_date::NumericDate;

#[derive(Debug)]
pub enum ClaimsRejection {
    AudienceMismatch {
        expected: Audience,
        found: AudienceClaim,
    },
    Expired {
        exp: NumericDate,
        now: NumericDate,
    },
    IssuerMismatch {
        expected: IssuerIdentifier,
        found: String,
    },
    Malformed {
        source: serde_json::Error,
    },
    NotYetValid {
        nbf: NumericDate,
        now: NumericDate,
    },
}

impl Display for ClaimsRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::AudienceMismatch { expected, found } => write!(
                formatter,
                "the token is meant for the audience {found} instead of exactly '{expected}'"
            ),
            Self::Expired { exp, now } => write!(
                formatter,
                "the token expired at {exp}, and the current time is {now}"
            ),
            Self::IssuerMismatch { expected, found } => write!(
                formatter,
                "the token is issued by '{found}' instead of '{expected}'"
            ),
            Self::Malformed { source } => {
                write!(formatter, "the token claims are malformed: {source}")
            }
            Self::NotYetValid { nbf, now } => write!(
                formatter,
                "the token is not valid before {nbf}, and the current time is {now}"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_registered_claims::audience::Audience;
    use margaret_registered_claims::audience_claim::AudienceClaim;
    use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
    use margaret_registered_claims::numeric_date::NumericDate;

    use super::ClaimsRejection;

    #[test]
    fn describes_every_rejection() {
        let described = [
            ClaimsRejection::AudienceMismatch {
                expected: "ours"
                    .parse::<Audience>()
                    .expect("the audience is not empty"),
                found: AudienceClaim::Multiple(vec!["ours".to_string(), "theirs".to_string()]),
            },
            ClaimsRejection::Expired {
                exp: NumericDate::new(100),
                now: NumericDate::new(150),
            },
            ClaimsRejection::IssuerMismatch {
                expected: "https://issuer.example"
                    .parse::<IssuerIdentifier>()
                    .expect("the issuer is an https url"),
                found: "https://attacker.example".to_string(),
            },
            ClaimsRejection::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            },
            ClaimsRejection::NotYetValid {
                nbf: NumericDate::new(200),
                now: NumericDate::new(150),
            },
        ]
        .map(|rejection| rejection.to_string());

        assert_eq!(
            described[0],
            "the token is meant for the audience [ours, theirs] instead of exactly 'ours'"
        );
        assert_eq!(
            described[1],
            "the token expired at 100, and the current time is 150"
        );
        assert_eq!(
            described[2],
            "the token is issued by 'https://attacker.example' instead of 'https://issuer.example'"
        );
        assert!(described[3].starts_with("the token claims are malformed: "));
        assert_eq!(
            described[4],
            "the token is not valid before 200, and the current time is 150"
        );
    }
}
