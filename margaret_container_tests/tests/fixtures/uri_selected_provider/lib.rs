pub trait TestStorage: Send + Sync {}

pub struct TestStorageUri;

pub fn resolve_test_storage(uri: TestStorageUri) -> std::sync::Arc<dyn TestStorage> {}

#[singleton]
struct Consumer {
    storage: std::sync::Arc<dyn TestStorage>,
}

impl Consumer {
    #[constructor]
    fn create(storage: std::sync::Arc<dyn TestStorage>) -> anyhow::Result<Self> {}
}
