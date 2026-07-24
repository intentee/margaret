use std::sync::Arc;

#[service]
struct Helper;

impl Helper {
    #[constructor]
    fn new() -> Self {}
}

#[service]
struct Worker;

impl Worker {
    #[constructor]
    fn new(helper: Arc<Helper>) -> Self {}
}
