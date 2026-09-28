use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[derive(Debug)]
pub enum ProviderMetadataRejection {
    IssuerMismatch {
        expected: IssuerIdentifier,
        found: String,
    },
    JwksUriMalformed {
        source: url::ParseError,
    },
    JwksUriNotHttps {
        scheme: String,
    },
    Malformed {
        source: serde_json::Error,
    },
}

impl Display for ProviderMetadataRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::IssuerMismatch { expected, found } => write!(
                formatter,
                "the provider metadata names the issuer '{found}' instead of '{expected}'"
            ),
            Self::JwksUriMalformed { source } => {
                write!(formatter, "the provider jwks_uri is not a url: {source}")
            }
            Self::JwksUriNotHttps { scheme } => write!(
                formatter,
                "the provider jwks_uri uses the '{scheme}' scheme instead of https"
            ),
            Self::Malformed { source } => {
                write!(formatter, "the provider metadata is malformed: {source}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

    use super::ProviderMetadataRejection;

    #[test]
    fn describes_every_rejection() {
        let described = [
            ProviderMetadataRejection::IssuerMismatch {
                expected: "https://issuer.example"
                    .parse::<IssuerIdentifier>()
                    .expect("the issuer is an https url"),
                found: "https://attacker.example".to_string(),
            },
            ProviderMetadataRejection::JwksUriMalformed {
                source: url::ParseError::EmptyHost,
            },
            ProviderMetadataRejection::JwksUriNotHttps {
                scheme: "http".to_string(),
            },
            ProviderMetadataRejection::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            },
        ]
        .map(|rejection| rejection.to_string());

        assert_eq!(
            described[0],
            "the provider metadata names the issuer 'https://attacker.example' instead of 'https://issuer.example'"
        );
        assert_eq!(
            described[1],
            "the provider jwks_uri is not a url: empty host"
        );
        assert_eq!(
            described[2],
            "the provider jwks_uri uses the 'http' scheme instead of https"
        );
        assert!(described[3].starts_with("the provider metadata is malformed: "));
    }
}
