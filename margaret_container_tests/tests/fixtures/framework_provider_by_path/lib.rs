use std::sync::Arc;

use crate::FrameworkStore;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(store: Arc<FrameworkStore>) -> anyhow::Result<Self> {}
}
