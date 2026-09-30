use std::collections::HashSet;
use std::sync::Arc;

use futures_util::future::join_all;
use tokio_util::sync::CancellationToken;

use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::issuer_directory_error::IssuerDirectoryError;
use crate::poll_issuer::poll_issuer;

pub struct IssuerDirectory {
    request_client: Arc<IssuerRequestClient>,
    trusted_issuers: Vec<Arc<TrustedIssuer>>,
}

impl IssuerDirectory {
    /// # Errors
    ///
    /// Returns `IssuerDirectoryError::IssuerTrustedTwice` when two declarations trust the same issuer.
    pub fn create(
        request_client: Arc<IssuerRequestClient>,
        trusted_issuers: Vec<Arc<TrustedIssuer>>,
    ) -> Result<Self, IssuerDirectoryError> {
        let mut issuers = HashSet::with_capacity(trusted_issuers.len());

        for trusted_issuer in &trusted_issuers {
            let issuer = &trusted_issuer.trust.token_trust().issuer;

            if !issuers.insert(issuer) {
                return Err(IssuerDirectoryError::IssuerTrustedTwice {
                    issuer: issuer.clone(),
                });
            }
        }

        Ok(Self {
            request_client,
            trusted_issuers,
        })
    }

    pub async fn run(&self, cancellation_token: CancellationToken) {
        join_all(self.trusted_issuers.iter().map(|trusted_issuer| {
            poll_issuer(&self.request_client, trusted_issuer, &cancellation_token)
        }))
        .await;
    }
}
