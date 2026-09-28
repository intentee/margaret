use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use reqwest::StatusCode;

use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;

#[derive(Debug)]
pub enum DiscoveryFailure {
    MetadataRejected(ProviderMetadataRejection),
    Status(StatusCode),
    Transport(reqwest::Error),
}

impl Display for DiscoveryFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::MetadataRejected(rejection) => {
                write!(formatter, "the provider metadata is rejected: {rejection}")
            }
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
