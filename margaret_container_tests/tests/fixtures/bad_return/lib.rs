use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Bad;

impl Bad {
    #[constructor]
    fn new() -> u8 {}
}
