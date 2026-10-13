use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Widget;

#[singleton]
struct Service {
    widget: pointers::Arc<Widget>,
}

impl Service {
    #[constructor]
    fn new(widget: pointers::Arc<Widget>) -> anyhow::Result<Self> {}
}
