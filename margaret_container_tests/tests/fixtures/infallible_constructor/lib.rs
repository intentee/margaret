use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Clock;

impl Clock {
    #[constructor]
    fn new() -> Self {}
}

#[singleton]
struct Greeter;

impl Greeter {
    #[constructor]
    fn new(clock: Arc<Clock>) -> Greeter {}
}
