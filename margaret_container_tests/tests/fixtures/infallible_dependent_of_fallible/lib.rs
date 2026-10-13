use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Loader;

impl Loader {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

#[singleton]
struct Reporter;

impl Reporter {
    #[constructor]
    fn new(loader: Arc<Loader>) -> Self {}
}
