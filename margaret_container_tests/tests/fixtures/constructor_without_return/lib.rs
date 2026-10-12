use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct MissingReturn;

impl MissingReturn {
    #[constructor]
    fn new() {}
}
