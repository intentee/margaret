use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::log_sink::LogSink;

#[singleton(collection = LogSink)]
pub struct StderrSink;

impl StderrSink {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        Self
    }
}

impl LogSink for StderrSink {
    fn write(&self, message: &str) {
        eprintln!("{message}");
    }
}
