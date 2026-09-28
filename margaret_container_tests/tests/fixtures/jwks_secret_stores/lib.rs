use std::sync::Arc;

use crate::AuthVerifier;
use crate::ServerStore;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(
        #[jwks_secret_store(server)] store: Arc<ServerStore>,
        #[jwks_secret_store(client = auth)] verifier: Arc<AuthVerifier>,
    ) -> anyhow::Result<Self> {}
}
