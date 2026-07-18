#[singleton]
struct Alpha;

impl Alpha {
    #[constructor]
    fn new(beta: Arc<Beta>) -> Self {}
}

#[singleton]
struct Beta;

impl Beta {
    #[constructor]
    fn new(alpha: Arc<Alpha>) -> Self {}
}
