use std::sync::Arc;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(#[jwks_secret_store] store: Arc<ServerCapability>) -> anyhow::Result<Self> {}
}
