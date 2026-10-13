use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Service;

impl Service {
    #[constructor]
    fn new(missing: Arc<Unprovided>) -> anyhow::Result<Self> {}
}
