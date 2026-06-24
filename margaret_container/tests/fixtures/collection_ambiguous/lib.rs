mod first {
    trait Plugin {}
}

mod second {
    trait Plugin {}
}

#[singleton(collection = Plugin)]
struct Member;

impl Member {
    #[constructor]
    fn new() -> Self {}
}
