use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::FrameworkStore;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(store: Arc<FrameworkStore>) -> anyhow::Result<Self> {}
}
