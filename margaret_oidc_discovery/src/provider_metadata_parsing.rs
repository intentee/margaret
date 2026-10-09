use std::sync::Arc;

use crate::provider_metadata::ProviderMetadata;
use crate::provider_metadata_rejection::ProviderMetadataRejection;

pub enum ProviderMetadataParsing {
    Accepted(Arc<ProviderMetadata>),
    Rejected(ProviderMetadataRejection),
}
