use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::service;
use margaret::framework::macros::singleton;

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
