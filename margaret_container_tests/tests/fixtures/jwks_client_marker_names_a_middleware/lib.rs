use std::sync::Arc;

use crate::AuthVerifier;

#[handles_middleware_attribute(attribute = logged)]
struct Logger;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(#[jwks_secret_store(client = logged)] verifier: Arc<AuthVerifier>) -> anyhow::Result<Self> {}
}
