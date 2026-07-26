use std::sync::Arc;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(dropped: Arc<crate::Dropped>) -> anyhow::Result<Self> {}
}
