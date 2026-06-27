trait Service {}
trait Other {}

#[provider(provides = Service)]
struct ServiceProvider;

impl ServiceProvider {
    #[constructor]
    fn new(other: Arc<dyn Other>) -> Self {}

    #[provide]
    fn provide(&self) -> Arc<dyn Service> {}
}

#[singleton(provides = Other)]
struct OtherImpl;

impl OtherImpl {
    #[constructor]
    fn new(service: Arc<dyn Service>) -> Self {}
}
