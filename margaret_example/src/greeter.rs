pub trait Greeter: Send + Sync {
    fn greet(&self) -> String;
}
