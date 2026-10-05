use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use reqwest::StatusCode;

use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;
use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;

#[derive(Debug)]
pub(crate) enum DiscoveryFailure {
    Exchange(IssuerExchangeError),
    MetadataRejected(ProviderMetadataRejection),
    Status(StatusCode),
}

impl Display for DiscoveryFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Exchange(failure) => write!(
                formatter,
                "the provider metadata could not be fetched: {failure}"
            ),
            Self::MetadataRejected(rejection) => {
                write!(formatter, "the provider metadata is rejected: {rejection}")
            }
            Self::Status(status) => write!(
                formatter,
                "the issuer answered the discovery request with status {status}"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;

    use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;
    use margaret_oidc_discovery::metadata_endpoint::MetadataEndpoint;
    use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;

    use super::DiscoveryFailure;

    #[test]
    fn describes_every_failure() {
        let described = [
            DiscoveryFailure::Exchange(IssuerExchangeError::Oversized { max_bytes: 16 }),
            DiscoveryFailure::MetadataRejected(ProviderMetadataRejection::EndpointNotHttps {
                endpoint: MetadataEndpoint::JwksUri,
                scheme: "http".to_string(),
            }),
            DiscoveryFailure::Status(StatusCode::NOT_FOUND),
        ]
        .map(|failure| failure.to_string());

        assert_eq!(
            described[0],
            "the provider metadata could not be fetched: the issuer answered with more than 16 bytes"
        );
        assert_eq!(
            described[1],
            "the provider metadata is rejected: the provider jwks_uri uses the 'http' scheme instead of https"
        );
        assert_eq!(
            described[2],
            "the issuer answered the discovery request with status 404 Not Found"
        );
    }
}
