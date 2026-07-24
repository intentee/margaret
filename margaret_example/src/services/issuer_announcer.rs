use std::convert::Infallible;
use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::service;

use crate::log_sink::LogSink;

#[service]
pub struct IssuerAnnouncer {
    issuer_endpoint: Arc<dyn ProvidesEndpoint>,
    sinks: Vec<Arc<dyn LogSink>>,
}

impl IssuerAnnouncer {
    #[constructor]
    #[must_use]
    pub fn create(
        #[endpoint_provider(issuer)] issuer_endpoint: Arc<dyn ProvidesEndpoint>,
        sinks: Vec<Arc<dyn LogSink>>,
    ) -> Self {
        Self {
            issuer_endpoint,
            sinks,
        }
    }

    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> Result<(), Infallible> {
        match self.issuer_endpoint.provide().await {
            Ok(issuer) => self.announce(&format!("resolved the issuer endpoint: {issuer}")),
            Err(error) => {
                self.announce(&format!("could not resolve the issuer endpoint: {error}"));
            }
        }

        cancellation_token.cancelled().await;

        Ok(())
    }

    fn announce(&self, message: &str) {
        for sink in &self.sinks {
            sink.write(message);
        }
    }
}
