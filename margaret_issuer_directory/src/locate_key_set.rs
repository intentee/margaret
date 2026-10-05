use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_issuer_request::issuer_document::IssuerDocument;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;

use crate::discovery_failure::DiscoveryFailure;
use crate::group_locator::GroupLocator;
use crate::issuer_group::IssuerGroup;
use crate::key_set_location::KeySetLocation;
use crate::key_set_location_failure::KeySetLocationFailure;

fn discovery_failure(failure: DiscoveryFailure) -> KeySetLocation {
    KeySetLocation::Failed(KeySetLocationFailure::Discovery(failure))
}

pub(crate) async fn locate_key_set(
    client: &IssuerRequestClient,
    group: &IssuerGroup,
    cancellation_token: &CancellationToken,
) -> KeySetLocation {
    match &group.locator {
        GroupLocator::Discovery {
            discovery_url,
            metadata: holders,
        } => {
            match client
                .fetch_document(discovery_url.clone(), cancellation_token)
                .await
            {
                IssuerDocument::Cancelled => KeySetLocation::Cancelled,
                IssuerDocument::Fetched(document) => {
                    match ProviderMetadata::parse(&document, group.issuer()) {
                        ProviderMetadataParsing::Accepted(metadata) => {
                            for holder in holders {
                                holder.hold(Arc::clone(&metadata));
                            }

                            KeySetLocation::Located(metadata.jwks_uri.clone())
                        }
                        ProviderMetadataParsing::Rejected(rejection) => {
                            discovery_failure(DiscoveryFailure::MetadataRejected(rejection))
                        }
                    }
                }
                IssuerDocument::Failed(failure) => {
                    discovery_failure(DiscoveryFailure::Exchange(failure))
                }
                IssuerDocument::UnexpectedStatus(status) => {
                    discovery_failure(DiscoveryFailure::Status(status))
                }
            }
        }
        GroupLocator::Endpoint(endpoint) => tokio::select! {
            biased;
            () = cancellation_token.cancelled() => KeySetLocation::Cancelled,
            provided = endpoint.provide() => match provided {
                Ok(key_set_url) => KeySetLocation::Located(key_set_url),
                Err(source) => KeySetLocation::Failed(KeySetLocationFailure::Endpoint(source)),
            },
        },
    }
}
