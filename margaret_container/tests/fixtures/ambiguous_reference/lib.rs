mod first {
    trait Plugin {}
}

mod second {
    trait Plugin {}
}

#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(plugins: Vec<Arc<dyn Plugin>>) -> Self {}
}
