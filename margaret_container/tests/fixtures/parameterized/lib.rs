#[singleton]
struct Plain;

impl Plain {
    #[constructor]
    fn new() -> Self {}
}

#[singleton]
struct Concrete {
    plain: Arc<Plain>,
    value: String,
}

impl Concrete {
    #[constructor]
    fn new(plain: Arc<Plain>, #[input] value: String) -> Self {}
}

trait Handler {}

#[singleton(provides = Handler)]
struct Interfaced {
    label: String,
}

impl Interfaced {
    #[constructor]
    fn new(#[input] label: String) -> Self {}
}
