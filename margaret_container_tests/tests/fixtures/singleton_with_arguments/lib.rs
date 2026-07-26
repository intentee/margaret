#[singleton(unexpected = Thing)]
struct Service;

impl Service {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}
