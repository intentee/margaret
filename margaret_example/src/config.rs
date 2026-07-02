use margaret_macros::constructor;
use margaret_macros::singleton;

#[singleton]
pub struct Config {
    app_name: String,
}

impl Config {
    #[constructor]
    pub fn create() -> Self {
        Self {
            app_name: "margaret".to_string(),
        }
    }

    pub fn app_name(&self) -> &str {
        &self.app_name
    }
}
