#[singleton]
struct Service;

impl Service {
    #[constructor]
    fn new(missing: Arc<Unprovided>) -> anyhow::Result<Self> {}
}
