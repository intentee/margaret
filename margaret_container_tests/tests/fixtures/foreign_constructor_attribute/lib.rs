use margaret::framework::macros::singleton;

#[singleton]
struct Lonely {
    value: String,
}

impl Lonely {
    #[other::constructor]
    fn new() -> anyhow::Result<Self> {}
}
