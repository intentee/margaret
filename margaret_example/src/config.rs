use margaret_macros::constructor;
use margaret_macros::singleton;

#[singleton]
pub struct Config {
    app_name: String,
}

impl Config {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        Self {
            app_name: "margaret".to_string(),
        }
    }

    #[must_use]
    pub fn app_name(&self) -> &str {
        &self.app_name
    }
}
