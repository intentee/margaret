use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_registered_claims::numeric_date::NumericDate;

#[derive(Debug)]
pub enum ClaimsRejection {
    Expired { exp: NumericDate, now: NumericDate },
    Malformed { source: serde_json::Error },
    NotYetValid { nbf: NumericDate, now: NumericDate },
}

impl Display for ClaimsRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Expired { exp, now } => write!(
                formatter,
                "the token expired at {exp}, and the current time is {now}"
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
    use margaret_registered_claims::numeric_date::NumericDate;

    use super::ClaimsRejection;

    #[test]
    fn describes_every_rejection() {
        let described = [
            ClaimsRejection::Expired {
                exp: NumericDate::new(100),
                now: NumericDate::new(150),
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
            "the token expired at 100, and the current time is 150"
        );
        assert!(described[1].starts_with("the token claims are malformed: "));
        assert_eq!(
            described[2],
            "the token is not valid before 200, and the current time is 150"
        );
    }
}
