use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::stores::Store;

mod stores {
    use margaret::framework::macros::constructor;
    use margaret::framework::macros::singleton;

    mod store {
    use margaret::framework::macros::constructor;
    use margaret::framework::macros::singleton;

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
