#[singleton]
struct Twice;

impl Twice {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}

    #[constructor]
    fn create() -> anyhow::Result<Self> {}
}
