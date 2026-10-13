use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(accessed: Arc<crate::Accessed>) -> anyhow::Result<Self> {}
}
