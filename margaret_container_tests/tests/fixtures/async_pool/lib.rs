use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

#[singleton]
struct Pool;

impl Pool {
    #[constructor]
    async fn new(config: Arc<Config>) -> anyhow::Result<Self> {}
}
