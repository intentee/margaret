#[singleton]
struct Alpha;

impl Alpha {
    #[constructor]
    fn new(beta: Arc<Beta>) -> anyhow::Result<Self> {}
}

#[singleton]
struct Beta;

impl Beta {
    #[constructor]
    fn new(alpha: Arc<Alpha>) -> anyhow::Result<Self> {}
}
