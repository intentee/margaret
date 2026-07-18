#[singleton(= 5)]
struct Service;

impl Service {
    #[constructor]
    fn new() -> Self {}
}
