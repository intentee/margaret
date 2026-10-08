use std::sync::Arc;

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(accessed: Arc<crate::Accessed>) -> anyhow::Result<Self> {}
}
