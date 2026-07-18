#[singleton]
struct Service;

impl Service {
    #[constructor]
    fn new(&self) -> Self {}
}
