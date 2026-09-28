use std::sync::Arc;

use crate::AuthVerifier;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(#[jwks_secret_store(server)] store: Arc<AuthVerifier>) -> anyhow::Result<Self> {}
}
