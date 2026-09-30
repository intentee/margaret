use tokio_util::sync::CancellationToken;

use margaret_issuer_request::issuer_document::IssuerDocument;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_trusted_issuer::key_set_locator::KeySetLocator;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::discovery_failure::DiscoveryFailure;
use crate::key_set_location::KeySetLocation;
use crate::key_set_location_failure::KeySetLocationFailure;

fn discovery_failure(failure: DiscoveryFailure) -> KeySetLocation {
    KeySetLocation::Failed(KeySetLocationFailure::Discovery(failure))
}

pub(crate) async fn locate_key_set(
    client: &IssuerRequestClient,
    TrustedIssuer { locator, trust, .. }: &TrustedIssuer,
    cancellation_token: &CancellationToken,
) -> KeySetLocation {
    match locator {
        KeySetLocator::Discovery {
            discovery_url,
            metadata: issuer_metadata,
        } => {
            match client
                .fetch_document(discovery_url.clone(), cancellation_token)
                .await
            {
                IssuerDocument::Cancelled => KeySetLocation::Cancelled,
                IssuerDocument::Fetched(document) => {
                    match ProviderMetadata::parse(&document, &trust.token_trust().issuer) {
                        ProviderMetadataParsing::Accepted(metadata) => {
                            let key_set_url = metadata.jwks_uri.clone();

                            issuer_metadata.hold(metadata);

                            KeySetLocation::Located(key_set_url)
                        }
                        ProviderMetadataParsing::Rejected(rejection) => {
                            discovery_failure(DiscoveryFailure::MetadataRejected(rejection))
                        }
                    }
                }
                IssuerDocument::Oversized { max_bytes } => {
                    discovery_failure(DiscoveryFailure::Oversized { max_bytes })
                }
                IssuerDocument::TransportFailed(source) => {
                    discovery_failure(DiscoveryFailure::Transport(source))
                }
                IssuerDocument::UnexpectedStatus(status) => {
                    discovery_failure(DiscoveryFailure::Status(status))
                }
            }
        }
        KeySetLocator::Endpoint(endpoint) => tokio::select! {
            biased;
            () = cancellation_token.cancelled() => KeySetLocation::Cancelled,
            provided = endpoint.provide() => match provided {
                Ok(key_set_url) => KeySetLocation::Located(key_set_url),
                Err(source) => KeySetLocation::Failed(KeySetLocationFailure::Endpoint(source)),
            },
        },
    }
}
