use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::log_sink::LogSink;

#[singleton(collection = LogSink)]
pub struct StdoutSink;

impl StdoutSink {
    #[constructor]
    pub fn create() -> Self {
        Self
    }
}

impl LogSink for StdoutSink {
    fn write(&self, message: &str) {
        println!("{message}");
    }
}
