#[singleton]
#[provides_endpoint]
struct BadProvider;

impl BadProvider {
    #[constructor]
    fn new() -> Self {}
}
