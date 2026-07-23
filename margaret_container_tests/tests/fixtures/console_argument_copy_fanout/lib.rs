#[singleton]
struct Child;

impl Child {
    #[constructor]
    fn create(#[console_argument(from = "retries")] retries: u16) -> Self {}
}

#[singleton]
struct Parent;

impl Parent {
    #[constructor]
    fn create(
        #[console_argument(from = "retries")] retries: u16,
        child: std::sync::Arc<Child>,
    ) -> Self {
    }
}
