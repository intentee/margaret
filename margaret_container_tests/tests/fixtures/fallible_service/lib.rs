use std::sync::Arc;

#[singleton]
struct Loader;

impl Loader {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

#[service]
struct Worker;

impl Worker {
    #[constructor]
    fn new(loader: Arc<Loader>) -> anyhow::Result<Self> {}
}
