#[singleton]
struct Twice;

impl Twice {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}

    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}
