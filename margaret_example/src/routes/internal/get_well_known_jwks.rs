use std::sync::Arc;

use margaret_http::response::Response;
use margaret_jwks_roller_server::jwks_publication::JwksPublication;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[responds_to_http(method = "get", path = "/.well-known/jwks.json", server = "internal")]
pub struct GetWellKnownJwks {
    jwks_publication: Arc<JwksPublication>,
}

impl GetWellKnownJwks {
    #[constructor]
    #[must_use]
    pub fn create(jwks_publication: Arc<JwksPublication>) -> Self {
        Self { jwks_publication }
    }

    #[process]
    pub async fn respond(&self) -> Response {
        self.jwks_publication.public_jwks_handler().respond()
    }
}
