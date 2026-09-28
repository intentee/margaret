use std::sync::Arc;

use async_trait::async_trait;

use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_key_set_poll::key_set_location::KeySetLocation;
use margaret_key_set_poll::key_set_location_request::KeySetLocationRequest;
use margaret_key_set_poll::locates_key_set::LocatesKeySet;

pub struct EndpointKeySetLocator {
    pub endpoint_provider: Arc<dyn ProvidesEndpoint>,
}

#[async_trait]
impl LocatesKeySet for EndpointKeySetLocator {
    type Failure = anyhow::Error;

    async fn locate(
        &self,
        KeySetLocationRequest {
            cancellation_token, ..
        }: KeySetLocationRequest<'_>,
    ) -> KeySetLocation<anyhow::Error> {
        tokio::select! {
            biased;
            () = cancellation_token.cancelled() => KeySetLocation::Cancelled,
            provided = self.endpoint_provider.provide() => match provided {
                Ok(key_set_url) => KeySetLocation::Located(key_set_url),
                Err(failure) => KeySetLocation::Failed(failure),
            },
        }
    }
}
