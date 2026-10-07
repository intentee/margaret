use std::ops::ControlFlow;

use tokio_util::sync::CancellationToken;

use margaret_issuer_request::issuer_document::IssuerDocument;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

use crate::discover_key_set_location::discover_key_set_location;
use crate::discovered_key_set_location::DiscoveredKeySetLocation;
use crate::key_set_poll::KeySetPoll;
use crate::key_set_poll_failure::KeySetPollFailure;
use crate::key_set_source::KeySetSource;

async fn fetched_key_set_document(
    client: &IssuerRequestClient,
    source: &KeySetSource,
    cancellation_token: &CancellationToken,
) -> ControlFlow<KeySetPoll, IssuerDocument> {
    match source {
        KeySetSource::Discovered { issuer, metadata } => {
            match discover_key_set_location(client, *issuer, metadata, cancellation_token).await {
                DiscoveredKeySetLocation::Cancelled => ControlFlow::Break(KeySetPoll::Cancelled),
                DiscoveredKeySetLocation::Failed(failure) => {
                    ControlFlow::Break(KeySetPoll::Failed(KeySetPollFailure::Discovery(failure)))
                }
                DiscoveredKeySetLocation::Located(jwks_uri) => {
                    ControlFlow::Continue(client.fetch_document(jwks_uri, cancellation_token).await)
                }
            }
        }
        KeySetSource::Published(issuer) => ControlFlow::Continue(
            client
                .fetch_document(issuer.jwks_uri, cancellation_token)
                .await,
        ),
    }
}

pub(crate) async fn fetch_key_set(
    client: &IssuerRequestClient,
    source: &KeySetSource,
    cancellation_token: &CancellationToken,
) -> KeySetPoll {
    let document = match fetched_key_set_document(client, source, cancellation_token).await {
        ControlFlow::Continue(document) => document,
        ControlFlow::Break(poll) => return poll,
    };

    match document {
        IssuerDocument::Cancelled => KeySetPoll::Cancelled,
        IssuerDocument::Fetched(document) => match VerificationKeySet::parse(&document) {
            KeySetDocumentParsing::Accepted(accepted) => KeySetPoll::Fetched(accepted),
            KeySetDocumentParsing::Rejected(rejection) => {
                KeySetPoll::Failed(KeySetPollFailure::DocumentRejected(rejection))
            }
        },
        IssuerDocument::Failed(failure) => {
            KeySetPoll::Failed(KeySetPollFailure::DocumentExchange(failure))
        }
        IssuerDocument::UnexpectedStatus(status) => {
            KeySetPoll::Failed(KeySetPollFailure::DocumentStatus(status))
        }
    }
}
