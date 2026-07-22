use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::tickers::jwks_roller::JwksRoller;

#[singleton]
#[responds_to_http(method = "get", path = "/.well-known/jwks.json", server = "internal")]
pub struct GetWellKnownJwks {
    jwks_roller: Arc<JwksRoller>,
}

impl GetWellKnownJwks {
    #[constructor]
    #[must_use]
    pub fn create(jwks_roller: Arc<JwksRoller>) -> Self {
        Self { jwks_roller }
    }

    #[process]
    pub async fn respond(&self) -> Response {
        self.jwks_roller.jwk_public_set_handler().respond()
    }
}
