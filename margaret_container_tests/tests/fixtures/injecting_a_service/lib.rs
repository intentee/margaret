use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::service;
use margaret::framework::macros::singleton;

#[service]
struct Worker;

impl Worker {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(worker: Arc<Worker>) -> anyhow::Result<Self> {}
}
