use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Pool;

impl Pool {
    #[constructor]
    async fn new() -> Self {}
}
