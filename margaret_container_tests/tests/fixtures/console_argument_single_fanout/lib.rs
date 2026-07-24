#[singleton]
struct Mapper;

impl Mapper {
    #[constructor]
    fn create(#[console_argument(from = "mapper")] mapper: Option<String>) -> Self {}
}

#[singleton]
struct First;

impl First {
    #[constructor]
    fn create(mapper: std::sync::Arc<Mapper>) -> Self {}
}

#[singleton]
struct Second;

impl Second {
    #[constructor]
    fn create(mapper: std::sync::Arc<Mapper>) -> Self {}
}

#[singleton]
struct Third;

impl Third {
    #[constructor]
    fn create(mapper: std::sync::Arc<Mapper>) -> Self {}
}

#[singleton]
struct Root;

impl Root {
    #[constructor]
    fn create(
        first: std::sync::Arc<First>,
        second: std::sync::Arc<Second>,
        third: std::sync::Arc<Third>,
    ) -> Self {
    }
}
