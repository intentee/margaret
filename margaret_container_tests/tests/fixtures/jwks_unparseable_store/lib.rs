use std::sync::Arc;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(#[jwks_secret_store(= 5)] store: Arc<ServerCapability>) -> Self {}
}
