trait Greeter {}
trait Missing {}

#[provider(provides = Greeter)]
struct GreeterProvider;

impl GreeterProvider {
    #[constructor]
    fn new(missing: Arc<dyn Missing>) -> Self {}

    #[provide]
    fn provide(&self) -> Arc<dyn Greeter> {}
}
