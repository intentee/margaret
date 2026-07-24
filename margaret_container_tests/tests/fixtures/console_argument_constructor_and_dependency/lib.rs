#[singleton]
struct Child;

impl Child {
    #[constructor]
    fn create(#[console_argument(from = "mapper")] mapper: Option<String>) -> Self {}
}

#[singleton]
struct Parent;

impl Parent {
    #[constructor]
    fn create(
        #[console_argument(from = "mapper")] mapper: Option<String>,
        child: std::sync::Arc<Child>,
    ) -> Self {
    }
}
