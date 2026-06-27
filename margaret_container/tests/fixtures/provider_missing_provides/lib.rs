#[provider]
struct Bad;

impl Bad {
    #[provide]
    fn provide(&self) -> Arc<dyn Greeter> {}
}
