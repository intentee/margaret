use std::sync::Arc;

use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::config::Config;
use crate::greeter::Greeter;

#[singleton(provides = Greeter)]
pub struct EnglishGreeter {
    config: Arc<Config>,
}

impl EnglishGreeter {
    #[constructor]
    #[must_use]
    pub fn create(config: Arc<Config>) -> Self {
        Self { config }
    }
}

impl Greeter for EnglishGreeter {
    fn greet(&self) -> String {
        format!("hello, {}", self.config.app_name())
    }
}
