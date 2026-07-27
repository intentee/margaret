use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use super::result::AppResult;

#[singleton]
pub struct Secrets {
    token: String,
}

impl Secrets {
    #[constructor]
    pub fn create() -> AppResult<Self> {
        Ok(Self {
            token: "sealed".to_string(),
        })
    }

    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }
}
