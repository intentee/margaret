use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::margaret::jwks::PublicJwksHandler;

#[singleton]
#[responds_to_http(method = "get", path = "/.well-known/jwks.json", server = "internal")]
pub struct GetWellKnownJwks {
    public_jwks_handler: Arc<PublicJwksHandler>,
}

impl GetWellKnownJwks {
    #[constructor]
    #[must_use]
    pub fn create(public_jwks_handler: Arc<PublicJwksHandler>) -> Self {
        Self {
            public_jwks_handler,
        }
    }

    #[process]
    pub async fn respond(&self) -> anyhow::Result<Response> {
        Ok(self.public_jwks_handler.respond())
    }
}
