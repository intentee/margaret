use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton(unexpected = Thing)]
struct Service;

impl Service {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}
