use anyhow::Result;
use reqwest::ClientBuilder;
use tokio_util::sync::CancellationToken;
use trzcina::Service as _;

use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;

use crate::key_set_poll_error::KeySetPollError;
use crate::key_set_poll_service::KeySetPollService;
use crate::locates_key_set::LocatesKeySet;
use crate::verification_key_set_holder::VerificationKeySetHolder;

/// # Errors
///
/// Returns `KeySetPollError::ClientBuild` when the issuer document client cannot be built.
pub async fn poll_key_set<TLocator: LocatesKeySet>(
    locator: TLocator,
    verification_key_set_holder: VerificationKeySetHolder,
    client_builder: ClientBuilder,
    cancellation_token: CancellationToken,
) -> Result<()> {
    let issuer_document_client = IssuerDocumentClient::build(client_builder)
        .map_err(|source| KeySetPollError::ClientBuild { source })?;

    Box::new(KeySetPollService {
        issuer_document_client,
        locator,
        verification_key_set_holder,
    })
    .run(cancellation_token)
    .await
}
