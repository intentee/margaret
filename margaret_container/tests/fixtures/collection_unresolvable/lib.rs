#[singleton(collection = Nonexistent)]
struct Member;

impl Member {
    #[constructor]
    fn new() -> Self {}
}
