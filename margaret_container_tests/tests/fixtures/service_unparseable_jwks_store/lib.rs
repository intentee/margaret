use std::sync::Arc;

#[service]
struct Worker;

impl Worker {
    #[constructor]
    fn new(#[jwks_secret_store(= 5)] store: Arc<ServerCapability>) -> anyhow::Result<Self> {}

    #[process]
    fn run(&self) -> anyhow::Result<()> {}
}
