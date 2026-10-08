use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::oidc_provider::ProviderMetadataHandler;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Get,
    path = "/.well-known/openid-configuration",
    server = "public"
)]
pub struct GetDiscovery {
    provider_metadata_handler: Arc<ProviderMetadataHandler>,
}

impl GetDiscovery {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(provider_metadata_handler: Arc<ProviderMetadataHandler>) -> anyhow::Result<Self> {
        Ok(Self {
            provider_metadata_handler,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self) -> anyhow::Result<Response> {
        Ok(self.provider_metadata_handler.respond())
    }
}
