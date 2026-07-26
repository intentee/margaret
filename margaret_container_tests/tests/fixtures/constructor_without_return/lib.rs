#[singleton]
struct MissingReturn;

impl MissingReturn {
    #[constructor]
    fn new() {}
}
