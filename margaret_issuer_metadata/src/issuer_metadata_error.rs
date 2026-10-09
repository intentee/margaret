use thiserror::Error;

use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;

#[derive(Debug, Error)]
pub enum IssuerMetadataError {
    #[error("the endpoints of the own provider do not describe its metadata: {rejection}")]
    OwnProviderEndpoints {
        rejection: ProviderMetadataRejection,
    },
}
