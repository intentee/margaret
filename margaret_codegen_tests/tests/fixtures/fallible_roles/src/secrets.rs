use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct Secrets {
    token: String,
}

impl Secrets {
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            token: "sealed".to_string(),
        })
    }

    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }
}
