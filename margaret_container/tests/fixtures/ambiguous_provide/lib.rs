trait Greeter {}

#[provider(provides = Greeter)]
struct Bad;

impl Bad {
    #[provide]
    fn one(&self) -> Arc<dyn Greeter> {}

    #[provide]
    fn two(&self) -> Arc<dyn Greeter> {}
}
