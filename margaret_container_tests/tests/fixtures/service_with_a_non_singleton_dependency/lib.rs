use std::sync::Arc;

#[service]
struct Helper;

impl Helper {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

#[service]
struct Worker;

impl Worker {
    #[constructor]
    fn new(helper: Arc<Helper>) -> anyhow::Result<Self> {}
}
