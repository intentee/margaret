use std::sync::Arc;

use async_trait::async_trait;
use url::Url;

use margaret_issuer_document_fetch::issuer_document_fetch::IssuerDocumentFetch;
use margaret_issuer_document_fetch::issuer_document_request::IssuerDocumentRequest;
use margaret_key_set_poll::key_set_location::KeySetLocation;
use margaret_key_set_poll::key_set_location_request::KeySetLocationRequest;
use margaret_key_set_poll::locates_key_set::LocatesKeySet;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_token_trust::declares_token_trust::DeclaresTokenTrust;

use crate::discovery_failure::DiscoveryFailure;

pub struct DiscoveryKeySetLocator {
    pub discovery_url: Url,
    pub token_trust: Arc<dyn DeclaresTokenTrust>,
}

#[async_trait]
impl LocatesKeySet for DiscoveryKeySetLocator {
    type Failure = DiscoveryFailure;

    async fn locate(
        &self,
        KeySetLocationRequest {
            cancellation_token,
            issuer_document_client,
            timeout,
        }: KeySetLocationRequest<'_>,
    ) -> KeySetLocation<DiscoveryFailure> {
        let document = match issuer_document_client
            .fetch(IssuerDocumentRequest {
                cancellation_token,
                timeout,
                url: self.discovery_url.clone(),
            })
            .await
        {
            IssuerDocumentFetch::Cancelled => return KeySetLocation::Cancelled,
            IssuerDocumentFetch::Fetched(document) => document,
            IssuerDocumentFetch::TransportFailed(source) => {
                return KeySetLocation::Failed(DiscoveryFailure::Transport(source));
            }
            IssuerDocumentFetch::UnexpectedStatus(status) => {
                return KeySetLocation::Failed(DiscoveryFailure::Status(status));
            }
        };

        match ProviderMetadata::parse(&document, &self.token_trust.token_trust().issuer) {
            ProviderMetadataParsing::Accepted(metadata) => {
                KeySetLocation::Located(metadata.jwks_uri().clone())
            }
            ProviderMetadataParsing::Rejected(rejection) => {
                KeySetLocation::Failed(DiscoveryFailure::MetadataRejected(rejection))
            }
        }
    }
}
