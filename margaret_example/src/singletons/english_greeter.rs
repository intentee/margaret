use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::config::app_name::AppName;

#[singleton]
pub struct EnglishGreeter {
    app_name: Arc<AppName>,
}

impl EnglishGreeter {
    #[must_use]
    #[constructor]
    pub fn create(app_name: Arc<AppName>) -> Self {
        Self { app_name }
    }

    #[must_use]
    pub fn greet(&self) -> String {
        format!("hello, {}", self.app_name.as_str())
    }
}
