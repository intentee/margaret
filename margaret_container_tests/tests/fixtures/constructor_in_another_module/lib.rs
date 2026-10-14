use std::sync::Arc;

use margaret::framework::macros::singleton;

use crate::decoys::English;

mod decoys {
    use margaret::framework::macros::singleton;

    #[singleton]
    pub struct English;
}

mod languages {
    use margaret::framework::macros::singleton;

    #[singleton]
    pub struct English;
}

#[singleton]
struct Greeter {
    english: Arc<English>,
}

mod construction {
    use std::sync::Arc;

    use margaret::framework::macros::constructor;

    use crate::Greeter;
    use crate::languages::English;

    impl Greeter {
        #[constructor]
        fn create(english: Arc<English>) -> anyhow::Result<Self> {}
    }
}
