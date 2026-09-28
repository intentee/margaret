use crate::provider_metadata::ProviderMetadata;
use crate::provider_metadata_rejection::ProviderMetadataRejection;

pub enum ProviderMetadataParsing {
    Accepted(ProviderMetadata),
    Rejected(ProviderMetadataRejection),
}
