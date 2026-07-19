use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::jwks_rolling::JwksRolling;

#[singleton]
#[responds_to_http(method = "get", path = "/.well-known/jwks.json", server = "internal")]
pub struct GetWellKnownJwks {
    jwks_rolling: Arc<JwksRolling>,
}

impl GetWellKnownJwks {
    #[constructor]
    pub fn create(jwks_rolling: Arc<JwksRolling>) -> Self {
        Self { jwks_rolling }
    }

    #[process]
    pub async fn respond(&self) -> Response {
        self.jwks_rolling.jwk_public_set_handler().respond()
    }
}
