use std::sync::Arc;

use futures_util::future::join_all;
use tokio_util::sync::CancellationToken;

use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::issuer_directory_error::IssuerDirectoryError;
use crate::issuer_group::IssuerGroup;
use crate::poll_issuer::poll_issuer;

pub struct IssuerDirectory {
    groups: Vec<IssuerGroup>,
    request_client: Arc<IssuerRequestClient>,
}

impl IssuerDirectory {
    /// # Errors
    ///
    /// Returns `IssuerDirectoryError::TokenTrustDeclaredTwice` when two declarations trust one
    /// issuer for one audience, and `IssuerDirectoryError::JwksEndpointIssuerTrustedTwice` when an
    /// issuer trusted through a jwks endpoint is trusted by another declaration as well.
    pub fn create(
        request_client: Arc<IssuerRequestClient>,
        trusted_issuers: Vec<Arc<TrustedIssuer>>,
    ) -> Result<Self, IssuerDirectoryError> {
        let mut groups: Vec<IssuerGroup> = Vec::new();

        for trusted_issuer in trusted_issuers {
            match groups
                .iter_mut()
                .find(|group| *group.issuer() == trusted_issuer.trust.token_trust().issuer)
            {
                Some(group) => group.admit(trusted_issuer)?,
                None => groups.push(IssuerGroup::founded_by(trusted_issuer)),
            }
        }

        Ok(Self {
            groups,
            request_client,
        })
    }

    pub async fn run(&self, cancellation_token: CancellationToken) {
        join_all(
            self.groups
                .iter()
                .map(|group| poll_issuer(&self.request_client, group, &cancellation_token)),
        )
        .await;
    }
}
