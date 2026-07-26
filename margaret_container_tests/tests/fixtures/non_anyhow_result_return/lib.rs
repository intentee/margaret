#[singleton]
struct Bad;

impl Bad {
    #[constructor]
    fn new() -> Result<Self, std::io::Error> {}
}
