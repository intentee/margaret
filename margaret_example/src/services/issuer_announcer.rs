use std::convert::Infallible;
use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::service;

#[service]
pub struct IssuerAnnouncer {
    issuer_endpoint: Arc<dyn ProvidesEndpoint>,
}

impl IssuerAnnouncer {
    #[constructor]
    #[must_use]
    pub fn create(#[endpoint_provider(issuer)] issuer_endpoint: Arc<dyn ProvidesEndpoint>) -> Self {
        Self { issuer_endpoint }
    }

    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> Result<(), Infallible> {
        match self.issuer_endpoint.provide().await {
            Ok(issuer) => println!("resolved the issuer endpoint: {issuer}"),
            Err(error) => println!("could not resolve the issuer endpoint: {error}"),
        }

        cancellation_token.cancelled().await;

        Ok(())
    }
}
