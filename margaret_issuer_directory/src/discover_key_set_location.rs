use tokio_util::sync::CancellationToken;

use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_document::IssuerDocument;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;

use crate::discovered_issuer::DiscoveredIssuer;
use crate::discovered_key_set_location::DiscoveredKeySetLocation;
use crate::discovery_failure::DiscoveryFailure;

pub(crate) async fn discover_key_set_location(
    client: &IssuerRequestClient,
    DiscoveredIssuer {
        discovery_url,
        issuer,
    }: DiscoveredIssuer,
    metadata: &IssuerMetadata,
    cancellation_token: &CancellationToken,
) -> DiscoveredKeySetLocation {
    match client
        .fetch_document(discovery_url, cancellation_token)
        .await
    {
        IssuerDocument::Cancelled => DiscoveredKeySetLocation::Cancelled,
        IssuerDocument::Fetched(document) => match ProviderMetadata::parse(&document, issuer) {
            ProviderMetadataParsing::Accepted(discovered) => {
                let jwks_uri = discovered.jwks_uri.clone();

                metadata.hold(discovered);

                DiscoveredKeySetLocation::Located(jwks_uri)
            }
            ProviderMetadataParsing::Rejected(rejection) => {
                DiscoveredKeySetLocation::Failed(DiscoveryFailure::MetadataRejected(rejection))
            }
        },
        IssuerDocument::Failed(failure) => {
            DiscoveredKeySetLocation::Failed(DiscoveryFailure::Exchange(failure))
        }
        IssuerDocument::Oversized { max_bytes } => {
            DiscoveredKeySetLocation::Failed(DiscoveryFailure::Oversized { max_bytes })
        }
        IssuerDocument::UnexpectedStatus(status) => {
            DiscoveredKeySetLocation::Failed(DiscoveryFailure::Status(status))
        }
    }
}
