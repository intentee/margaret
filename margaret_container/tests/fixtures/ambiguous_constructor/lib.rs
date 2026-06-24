#[singleton]
struct Twice;

impl Twice {
    #[constructor]
    fn new() -> Self {}

    #[constructor]
    fn create() -> Self {}
}
