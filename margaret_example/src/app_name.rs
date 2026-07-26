use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct AppName {
    value: String,
}

impl AppName {
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok({
            Self {
                value: "margaret".to_string(),
            }
        })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}
