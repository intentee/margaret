#[singleton]
struct Service;

impl Service {
    #[constructor]
    fn new(bad: &str) -> Self {}
}
