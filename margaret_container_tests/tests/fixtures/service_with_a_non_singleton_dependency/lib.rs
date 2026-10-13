use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::service;

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
