use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::jwks::PublicJwksHandler;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/jwks.json", server = "identity")]
pub struct GetJwks {
    public_jwks_handler: Arc<PublicJwksHandler>,
}

impl GetJwks {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(public_jwks_handler: Arc<PublicJwksHandler>) -> anyhow::Result<Self> {
        Ok(Self {
            public_jwks_handler,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self) -> anyhow::Result<Response> {
        Ok(self.public_jwks_handler.respond())
    }
}
