#[singleton]
#[provides_endpoint(= 5)]
struct BadArgs;

impl BadArgs {
    #[constructor]
    fn new() -> Self {}
}
