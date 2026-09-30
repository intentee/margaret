use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::metadata_endpoint::MetadataEndpoint;

#[derive(Debug)]
pub enum ProviderMetadataRejection {
    EndpointMalformed {
        endpoint: MetadataEndpoint,
        source: url::ParseError,
    },
    EndpointNotHttps {
        endpoint: MetadataEndpoint,
        scheme: String,
    },
    IssuerMismatch {
        expected: IssuerIdentifier,
        found: String,
    },
    Malformed {
        source: serde_json::Error,
    },
}

impl Display for ProviderMetadataRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::EndpointMalformed { endpoint, source } => {
                write!(formatter, "the provider {endpoint} is not a url: {source}")
            }
            Self::EndpointNotHttps { endpoint, scheme } => write!(
                formatter,
                "the provider {endpoint} uses the '{scheme}' scheme instead of https"
            ),
            Self::IssuerMismatch { expected, found } => write!(
                formatter,
                "the provider metadata names the issuer '{found}' instead of '{expected}'"
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
    use crate::metadata_endpoint::MetadataEndpoint;

    #[test]
    fn describes_every_rejection() {
        let described = [
            ProviderMetadataRejection::EndpointMalformed {
                endpoint: MetadataEndpoint::JwksUri,
                source: url::ParseError::EmptyHost,
            },
            ProviderMetadataRejection::EndpointNotHttps {
                endpoint: MetadataEndpoint::TokenEndpoint,
                scheme: "http".to_string(),
            },
            ProviderMetadataRejection::IssuerMismatch {
                expected: "https://issuer.example"
                    .parse::<IssuerIdentifier>()
                    .expect("the issuer is an https url"),
                found: "https://attacker.example".to_string(),
            },
            ProviderMetadataRejection::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            },
        ]
        .map(|rejection| rejection.to_string());

        assert_eq!(
            described[0],
            "the provider jwks_uri is not a url: empty host"
        );
        assert_eq!(
            described[1],
            "the provider token_endpoint uses the 'http' scheme instead of https"
        );
        assert_eq!(
            described[2],
            "the provider metadata names the issuer 'https://attacker.example' instead of 'https://issuer.example'"
        );
        assert!(described[3].starts_with("the provider metadata is malformed: "));
    }
}
