use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use reqwest::StatusCode;

use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;

#[derive(Debug)]
pub(crate) enum DiscoveryFailure {
    MetadataRejected(ProviderMetadataRejection),
    Oversized { max_bytes: usize },
    Status(StatusCode),
    Transport(reqwest::Error),
}

impl Display for DiscoveryFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::MetadataRejected(rejection) => {
                write!(formatter, "the provider metadata is rejected: {rejection}")
            }
            Self::Oversized { max_bytes } => write!(
                formatter,
                "the provider metadata is larger than {max_bytes} bytes"
            ),
            Self::Status(status) => write!(
                formatter,
                "the issuer answered the discovery request with status {status}"
            ),
            Self::Transport(source) => write!(
                formatter,
                "the provider metadata could not be transferred from the issuer: {source}"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Client;
    use reqwest::StatusCode;

    use margaret_oidc_discovery::metadata_endpoint::MetadataEndpoint;
    use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;

    use super::DiscoveryFailure;

    #[test]
    fn describes_every_failure() {
        let described = [
            DiscoveryFailure::MetadataRejected(ProviderMetadataRejection::EndpointNotHttps {
                endpoint: MetadataEndpoint::JwksUri,
                scheme: "http".to_string(),
            }),
            DiscoveryFailure::Oversized { max_bytes: 16 },
            DiscoveryFailure::Status(StatusCode::NOT_FOUND),
            DiscoveryFailure::Transport(
                Client::new()
                    .get("https://")
                    .build()
                    .expect_err("an empty host is not a request"),
            ),
        ]
        .map(|failure| failure.to_string());

        assert_eq!(
            described[0],
            "the provider metadata is rejected: the provider jwks_uri uses the 'http' scheme instead of https"
        );
        assert_eq!(
            described[1],
            "the provider metadata is larger than 16 bytes"
        );
        assert_eq!(
            described[2],
            "the issuer answered the discovery request with status 404 Not Found"
        );
        assert!(
            described[3]
                .starts_with("the provider metadata could not be transferred from the issuer: ")
        );
    }
}
