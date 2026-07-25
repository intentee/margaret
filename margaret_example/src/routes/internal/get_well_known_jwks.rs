use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

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
    pub async fn respond(&self) -> Response {
        self.public_jwks_handler.respond()
    }
}
