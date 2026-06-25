pub trait Plugin: Send + Sync {
    fn name(&self) -> String;
}
