#[singleton]
struct Consumer {
    thing: Arc<baked_crate::BakedThing>,
}

impl Consumer {
    #[constructor]
    fn new(thing: Arc<baked_crate::BakedThing>) -> Self {}
}
