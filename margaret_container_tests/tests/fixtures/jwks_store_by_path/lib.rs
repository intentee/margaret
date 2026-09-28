use std::sync::Arc;

use crate::ServerStore;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(store: Arc<ServerStore>) -> anyhow::Result<Self> {}
}
