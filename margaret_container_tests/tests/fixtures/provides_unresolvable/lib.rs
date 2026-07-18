#[singleton(provides = Nonexistent)]
struct Service;

impl Service {
    #[constructor]
    fn new() -> Self {}
}
