use std::sync::Arc;

#[service]
struct Worker;

impl Worker {
    #[constructor]
    fn new() -> Self {}
}

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(worker: Arc<Worker>) -> Self {}
}
