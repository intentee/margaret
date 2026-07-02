pub trait LogSink: Send + Sync {
    fn write(&self, message: &str);
}
