use std::sync::Arc;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(
        #[jwks_secret_store(server)] store: Arc<ServerCapability>,
        #[jwks_secret_store(client = auth)] verifier: Arc<ClientVerifier>,
    ) -> Self {}
}
