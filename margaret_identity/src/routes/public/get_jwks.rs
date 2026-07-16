use std::sync::Arc;

use margaret_http::response::Response;
use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::stores::signing_key_store::SigningKeyStore;

#[singleton]
#[responds_to_http(method = "get", path = "/.well-known/jwks.json", server = "public")]
pub struct GetJwks {
    store: Arc<SigningKeyStore>,
}

impl GetJwks {
    #[constructor]
    pub fn create(store: Arc<SigningKeyStore>) -> Self {
        Self { store }
    }

    #[process]
    pub async fn respond(&self) -> Response {
        let Some(secret) = self.store.current() else {
            return Response::text(503, "the signing keys are not ready");
        };

        Response::json(200, &JwkPublicSet::from(secret))
    }
}
