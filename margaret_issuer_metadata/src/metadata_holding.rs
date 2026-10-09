use std::sync::Arc;

use margaret_oidc_discovery::provider_metadata::ProviderMetadata;

#[derive(Clone)]
pub enum MetadataHolding {
    Awaiting,
    Held(Arc<ProviderMetadata>),
}
