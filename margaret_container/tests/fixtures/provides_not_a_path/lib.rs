#[singleton(provides = "text")]
struct Service;

impl Service {
    #[constructor]
    fn new() -> Self {}
}
