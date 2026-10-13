use std::sync::Arc;

use tokio::sync::watch;

use margaret_oidc_discovery::provider_endpoints::ProviderEndpoints;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;

use crate::issuer_metadata_error::IssuerMetadataError;
use crate::metadata_holding::MetadataHolding;

pub struct IssuerMetadata {
    holding: watch::Sender<MetadataHolding>,
}

impl IssuerMetadata {
    #[must_use]
    pub fn awaiting() -> Self {
        Self {
            holding: watch::Sender::new(MetadataHolding::Awaiting),
        }
    }

    /// # Errors
    ///
    /// Returns `IssuerMetadataError::OwnProviderEndpoints` when an endpoint of the own provider
    /// is not an https url.
    pub fn of_provider(endpoints: ProviderEndpoints) -> Result<Self, IssuerMetadataError> {
        match ProviderMetadata::of_endpoints(&endpoints) {
            ProviderMetadataParsing::Accepted(metadata) => Ok(Self {
                holding: watch::Sender::new(MetadataHolding::Held(metadata)),
            }),
            ProviderMetadataParsing::Rejected(rejection) => {
                Err(IssuerMetadataError::OwnProviderEndpoints { rejection })
            }
        }
    }

    pub fn hold(&self, metadata: Arc<ProviderMetadata>) {
        self.holding.send_replace(MetadataHolding::Held(metadata));
    }

    #[must_use]
    pub fn holding(&self) -> MetadataHolding {
        self.holding.borrow().clone()
    }
}
