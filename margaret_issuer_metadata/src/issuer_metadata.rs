use std::sync::Arc;

use tokio::sync::watch;

use margaret_oidc_discovery::provider_metadata::ProviderMetadata;

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

    pub fn hold(&self, metadata: Arc<ProviderMetadata>) {
        self.holding.send_replace(MetadataHolding::Held(metadata));
    }

    #[must_use]
    pub fn holding(&self) -> MetadataHolding {
        self.holding.borrow().clone()
    }
}
