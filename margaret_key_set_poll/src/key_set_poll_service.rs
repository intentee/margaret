use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use log::error;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use trzcina::TickContext;
use trzcina::Ticker;

use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_issuer_document_fetch::issuer_document_fetch::IssuerDocumentFetch;
use margaret_issuer_document_fetch::issuer_document_request::IssuerDocumentRequest;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

use crate::key_set_location::KeySetLocation;
use crate::key_set_location_request::KeySetLocationRequest;
use crate::key_set_poll::KeySetPoll;
use crate::key_set_poll_failure::KeySetPollFailure;
use crate::key_set_poll_interval_after_ready::KEY_SET_POLL_INTERVAL_AFTER_READY;
use crate::key_set_poll_interval_before_ready::KEY_SET_POLL_INTERVAL_BEFORE_READY;
use crate::locates_key_set::LocatesKeySet;
use crate::verification_key_set_holder::VerificationKeySetHolder;

pub struct KeySetPollService<TLocator> {
    pub issuer_document_client: IssuerDocumentClient,
    pub locator: TLocator,
    pub verification_key_set_holder: VerificationKeySetHolder,
}

impl<TLocator: LocatesKeySet> KeySetPollService<TLocator> {
    pub async fn fetch_key_set(
        &self,
        cancellation_token: &CancellationToken,
        timeout: Duration,
    ) -> KeySetPoll<TLocator::Failure> {
        let key_set_url = match self
            .locator
            .locate(KeySetLocationRequest {
                cancellation_token,
                issuer_document_client: &self.issuer_document_client,
                timeout,
            })
            .await
        {
            KeySetLocation::Cancelled => return KeySetPoll::Cancelled,
            KeySetLocation::Failed(failure) => {
                return KeySetPoll::Failed(KeySetPollFailure::Location(failure));
            }
            KeySetLocation::Located(key_set_url) => key_set_url,
        };

        match self
            .issuer_document_client
            .fetch(IssuerDocumentRequest {
                cancellation_token,
                timeout,
                url: key_set_url,
            })
            .await
        {
            IssuerDocumentFetch::Cancelled => KeySetPoll::Cancelled,
            IssuerDocumentFetch::Fetched(document) => match VerificationKeySet::parse(&document) {
                KeySetDocumentParsing::Accepted(accepted) => KeySetPoll::Fetched(accepted),
                KeySetDocumentParsing::Rejected(rejection) => {
                    KeySetPoll::Failed(KeySetPollFailure::DocumentRejected(rejection))
                }
            },
            IssuerDocumentFetch::TransportFailed(source) => {
                KeySetPoll::Failed(KeySetPollFailure::DocumentTransport(source))
            }
            IssuerDocumentFetch::UnexpectedStatus(status) => {
                KeySetPoll::Failed(KeySetPollFailure::DocumentStatus(status))
            }
        }
    }

    fn poll_interval(&self) -> Duration {
        if self.verification_key_set_holder.is_ready() {
            KEY_SET_POLL_INTERVAL_AFTER_READY
        } else {
            KEY_SET_POLL_INTERVAL_BEFORE_READY
        }
    }
}

#[async_trait]
impl<TLocator: LocatesKeySet> Ticker for KeySetPollService<TLocator> {
    fn tick_interval(&self) -> Duration {
        KEY_SET_POLL_INTERVAL_BEFORE_READY
    }

    async fn handle_tick(
        &mut self,
        cancellation_token: CancellationToken,
        _tick_context: TickContext,
    ) -> Result<()> {
        match self
            .fetch_key_set(&cancellation_token, self.poll_interval())
            .await
        {
            KeySetPoll::Cancelled => return Ok(()),
            KeySetPoll::Failed(failure) => {
                error!("Unable to poll the key set of the issuer: {failure}");
            }
            KeySetPoll::Fetched(AcceptedKeySetDocument {
                ignored_keys,
                key_set,
            }) => {
                for ignored_key in ignored_keys {
                    debug!("Ignoring a key of the issuer's key set: {ignored_key}");
                }

                self.verification_key_set_holder
                    .set(Some(Arc::new(key_set)));
            }
        }

        if self.verification_key_set_holder.is_ready() {
            tokio::select! {
                biased;
                () = cancellation_token.cancelled() => {}
                () = sleep(KEY_SET_POLL_INTERVAL_AFTER_READY) => {}
            }
        }

        Ok(())
    }
}
