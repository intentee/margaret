#[singleton]
struct Bad;

impl Bad {
    #[constructor]
    fn new() -> u8 {}
}
