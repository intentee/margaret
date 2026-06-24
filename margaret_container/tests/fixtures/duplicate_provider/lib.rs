trait Greeter {}

#[singleton(provides = Greeter)]
struct English;

impl English {
    #[constructor]
    fn new() -> Self {}
}

#[singleton(provides = Greeter)]
struct French;

impl French {
    #[constructor]
    fn new() -> Self {}
}
