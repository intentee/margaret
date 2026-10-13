use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Service;

impl Service {
    #[constructor]
    fn new(&self) -> anyhow::Result<Self> {}
}
