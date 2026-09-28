use std::sync::Arc;

use crate::stores::Store;

mod stores {
    mod store {
        #[singleton]
        pub struct Store;
    }

    pub use store::Store;
}

#[singleton]
struct Consumer {
    store: Arc<Store>,
}

impl Consumer {
    #[constructor]
    fn create(store: Arc<Store>) -> anyhow::Result<Self> {}
}
