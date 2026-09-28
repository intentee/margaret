use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use log::error;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use trzcina::TickContext;
use trzcina::Ticker;

use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_issuer_document_fetch::issuer_document_fetch::IssuerDocumentFetch;
use margaret_issuer_document_fetch::issuer_document_request::IssuerDocumentRequest;
use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

use crate::jwks_client_error::JwksClientError;
use crate::jwks_poll_interval_after_ready::JWKS_POLL_INTERVAL_AFTER_READY;
use crate::jwks_poll_interval_before_ready::JWKS_POLL_INTERVAL_BEFORE_READY;
use crate::public_jwks_poll::PublicJwksPoll;
use crate::verification_key_set_holder::VerificationKeySetHolder;

pub struct PublicJwksPollService {
    pub endpoint_provider: Arc<dyn ProvidesEndpoint>,
    pub issuer_document_client: IssuerDocumentClient,
    pub verification_key_set_holder: VerificationKeySetHolder,
}

impl PublicJwksPollService {
    pub async fn fetch_public_jwks(
        &self,
        cancellation_token: &CancellationToken,
        timeout: Duration,
    ) -> PublicJwksPoll {
        let jwks_url = tokio::select! {
            biased;
            () = cancellation_token.cancelled() => return PublicJwksPoll::Cancelled,
            provided = self.endpoint_provider.provide() => match provided {
                Ok(jwks_url) => jwks_url,
                Err(error) => {
                    return PublicJwksPoll::Failed(JwksClientError::EndpointResolution(error));
                }
            },
        };

        match self
            .issuer_document_client
            .fetch(IssuerDocumentRequest {
                cancellation_token,
                timeout,
                url: jwks_url,
            })
            .await
        {
            IssuerDocumentFetch::Cancelled => PublicJwksPoll::Cancelled,
            IssuerDocumentFetch::Fetched(document) => match VerificationKeySet::parse(&document) {
                KeySetParsing::Accepted(key_set) => PublicJwksPoll::Fetched(key_set),
                KeySetParsing::Rejected(rejection) => {
                    PublicJwksPoll::Failed(JwksClientError::DocumentRejected { rejection })
                }
            },
            IssuerDocumentFetch::TransportFailed(source) => {
                PublicJwksPoll::Failed(JwksClientError::DocumentTransport { source })
            }
            IssuerDocumentFetch::UnexpectedStatus(status) => {
                PublicJwksPoll::Failed(JwksClientError::DocumentStatus { status })
            }
        }
    }

    fn poll_interval(&self) -> Duration {
        if self.verification_key_set_holder.is_ready() {
            JWKS_POLL_INTERVAL_AFTER_READY
        } else {
            JWKS_POLL_INTERVAL_BEFORE_READY
        }
    }
}

#[async_trait]
impl Ticker for PublicJwksPollService {
    fn tick_interval(&self) -> Duration {
        JWKS_POLL_INTERVAL_BEFORE_READY
    }

    async fn handle_tick(
        &mut self,
        cancellation_token: CancellationToken,
        _tick_context: TickContext,
    ) -> Result<()> {
        match self
            .fetch_public_jwks(&cancellation_token, self.poll_interval())
            .await
        {
            PublicJwksPoll::Cancelled => return Ok(()),
            PublicJwksPoll::Failed(error) => {
                error!("Unable to fetch the jwks document from the issuer: {error}");
            }
            PublicJwksPoll::Fetched(key_set) => {
                self.verification_key_set_holder
                    .set(Some(Arc::new(key_set)));
            }
        }

        if self.verification_key_set_holder.is_ready() {
            tokio::select! {
                biased;
                () = cancellation_token.cancelled() => {}
                () = sleep(JWKS_POLL_INTERVAL_AFTER_READY) => {}
            }
        }

        Ok(())
    }
}
