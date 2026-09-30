use tokio_util::sync::CancellationToken;

use margaret_issuer_request::issuer_document::IssuerDocument;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::key_set_location::KeySetLocation;
use crate::key_set_poll::KeySetPoll;
use crate::key_set_poll_failure::KeySetPollFailure;
use crate::locate_key_set::locate_key_set;

pub(crate) async fn fetch_key_set(
    client: &IssuerRequestClient,
    trusted_issuer: &TrustedIssuer,
    cancellation_token: &CancellationToken,
) -> KeySetPoll {
    let key_set_url = match locate_key_set(client, trusted_issuer, cancellation_token).await {
        KeySetLocation::Cancelled => return KeySetPoll::Cancelled,
        KeySetLocation::Failed(failure) => {
            return KeySetPoll::Failed(KeySetPollFailure::Location(failure));
        }
        KeySetLocation::Located(key_set_url) => key_set_url,
    };

    match client.fetch_document(key_set_url, cancellation_token).await {
        IssuerDocument::Cancelled => KeySetPoll::Cancelled,
        IssuerDocument::Fetched(document) => match VerificationKeySet::parse(&document) {
            KeySetDocumentParsing::Accepted(accepted) => KeySetPoll::Fetched(accepted),
            KeySetDocumentParsing::Rejected(rejection) => {
                KeySetPoll::Failed(KeySetPollFailure::DocumentRejected(rejection))
            }
        },
        IssuerDocument::Oversized { max_bytes } => {
            KeySetPoll::Failed(KeySetPollFailure::DocumentOversized { max_bytes })
        }
        IssuerDocument::TransportFailed(source) => {
            KeySetPoll::Failed(KeySetPollFailure::DocumentTransport(source))
        }
        IssuerDocument::UnexpectedStatus(status) => {
            KeySetPoll::Failed(KeySetPollFailure::DocumentStatus(status))
        }
    }
}
