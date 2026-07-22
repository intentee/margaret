use std::sync::Arc;

use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::app_name::AppName;
use crate::greeter::Greeter;

#[singleton(provides = Greeter)]
pub struct EnglishGreeter {
    app_name: Arc<AppName>,
}

impl EnglishGreeter {
    #[constructor]
    #[must_use]
    pub fn create(app_name: Arc<AppName>) -> Self {
        Self { app_name }
    }
}

impl Greeter for EnglishGreeter {
    fn greet(&self) -> String {
        format!("hello, {}", self.app_name.as_str())
    }
}
