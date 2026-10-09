use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_https_url::https_url_rejection::HttpsUrlRejection;

use crate::metadata_endpoint::MetadataEndpoint;

#[derive(Debug)]
pub enum ProviderMetadataRejection {
    Endpoint {
        endpoint: MetadataEndpoint,
        rejection: HttpsUrlRejection,
    },
    IssuerMismatch {
        expected: &'static str,
        found: String,
    },
    Malformed {
        source: serde_json::Error,
    },
}

impl Display for ProviderMetadataRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Endpoint {
                endpoint,
                rejection,
            } => write!(
                formatter,
                "the provider {endpoint} is rejected: {rejection}"
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
    use margaret_https_url::https_url_rejection::HttpsUrlRejection;

    use super::ProviderMetadataRejection;
    use crate::metadata_endpoint::MetadataEndpoint;

    #[test]
    fn describes_every_rejection() {
        let described = [
            ProviderMetadataRejection::Endpoint {
                endpoint: MetadataEndpoint::TokenEndpoint,
                rejection: HttpsUrlRejection::NotHttps {
                    scheme: "http".to_string(),
                },
            },
            ProviderMetadataRejection::IssuerMismatch {
                expected: "https://issuer.example",
                found: "https://attacker.example".to_string(),
            },
            ProviderMetadataRejection::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            },
        ]
        .map(|rejection| rejection.to_string());

        assert_eq!(
            described[0],
            "the provider token_endpoint is rejected: the url uses the 'http' scheme instead of https"
        );
        assert_eq!(
            described[1],
            "the provider metadata names the issuer 'https://attacker.example' instead of 'https://issuer.example'"
        );
        assert!(described[2].starts_with("the provider metadata is malformed: "));
    }
}
