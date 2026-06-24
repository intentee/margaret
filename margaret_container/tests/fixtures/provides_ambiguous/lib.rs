mod first {
    trait Greeter {}
}

mod second {
    trait Greeter {}
}

#[singleton(provides = Greeter)]
struct Service;

impl Service {
    #[constructor]
    fn new() -> Self {}
}
