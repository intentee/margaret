use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;

#[derive(Debug)]
pub enum ClaimsRejection {
    AudienceMismatch {
        expected: String,
        found: AudienceClaim,
    },
    AudienceNotSole {
        expected: String,
        found: AudienceClaim,
    },
    AudienceOutside {
        expected: Vec<String>,
        found: AudienceClaim,
    },
    Expired {
        exp: NumericDate,
        now: NumericDate,
    },
    IssuerMismatch {
        expected: String,
        found: String,
    },
    Malformed {
        source: serde_json::Error,
    },
    MissingIssuedAt,
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
                "the token's audience {found} does not include '{expected}'"
            ),
            Self::AudienceNotSole { expected, found } => write!(
                formatter,
                "the token's audience {found} is not '{expected}' alone"
            ),
            Self::AudienceOutside { expected, found } => write!(
                formatter,
                "the token's audience {found} includes none of [{}]",
                expected.join(", ")
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
            Self::MissingIssuedAt => {
                formatter.write_str("the token does not state when it was issued")
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
    use margaret_registered_claims::audience_claim::AudienceClaim;
    use margaret_registered_claims::numeric_date::NumericDate;

    use super::ClaimsRejection;

    #[test]
    fn describes_every_rejection() {
        let described = [
            ClaimsRejection::AudienceMismatch {
                expected: "ours".to_string(),
                found: AudienceClaim::Multiple(vec!["theirs".to_string(), "others".to_string()]),
            },
            ClaimsRejection::AudienceNotSole {
                expected: "ours".to_string(),
                found: AudienceClaim::Multiple(vec!["ours".to_string()]),
            },
            ClaimsRejection::AudienceOutside {
                expected: vec!["first".to_string(), "second".to_string()],
                found: AudienceClaim::Single("theirs".to_string()),
            },
            ClaimsRejection::Expired {
                exp: NumericDate::new(100),
                now: NumericDate::new(150),
            },
            ClaimsRejection::IssuerMismatch {
                expected: "https://issuer.example".to_string(),
                found: "https://attacker.example".to_string(),
            },
            ClaimsRejection::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            },
            ClaimsRejection::MissingIssuedAt,
            ClaimsRejection::NotYetValid {
                nbf: NumericDate::new(200),
                now: NumericDate::new(150),
            },
        ]
        .map(|rejection| rejection.to_string());

        assert_eq!(
            described[0],
            "the token's audience [theirs, others] does not include 'ours'"
        );
        assert_eq!(
            described[1],
            "the token's audience [ours] is not 'ours' alone"
        );
        assert_eq!(
            described[2],
            "the token's audience theirs includes none of [first, second]"
        );
        assert_eq!(
            described[3],
            "the token expired at 100, and the current time is 150"
        );
        assert_eq!(
            described[4],
            "the token is issued by 'https://attacker.example' instead of 'https://issuer.example'"
        );
        assert!(described[5].starts_with("the token claims are malformed: "));
        assert_eq!(described[6], "the token does not state when it was issued");
        assert_eq!(
            described[7],
            "the token is not valid before 200, and the current time is 150"
        );
    }
}
