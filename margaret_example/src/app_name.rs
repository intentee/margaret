use margaret_macros::constructor;
use margaret_macros::singleton;

#[singleton]
pub struct AppName {
    value: String,
}

impl AppName {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        Self {
            value: "margaret".to_string(),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}
