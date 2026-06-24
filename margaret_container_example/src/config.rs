use margaret_macros::constructor;
use margaret_macros::singleton;

#[singleton]
pub struct Config {
    pub message: String,
}

impl Config {
    #[constructor]
    pub fn create() -> Self {
        Self {
            message: "hello from margaret".to_string(),
        }
    }
}
