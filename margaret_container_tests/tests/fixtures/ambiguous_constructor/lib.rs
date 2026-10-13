use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Twice;

impl Twice {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}

    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}
