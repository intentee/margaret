pub trait Clock: Send + Sync {
    fn revision(&self) -> String;
}
