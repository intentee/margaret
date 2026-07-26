use std::sync::Arc;

#[singleton]
struct Loader;

impl Loader {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(loader: Arc<Loader>) -> Self {}
}
