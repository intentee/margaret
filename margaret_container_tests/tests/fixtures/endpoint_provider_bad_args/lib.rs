trait ProvidesEndpoint {}

#[singleton]
struct Poller;

impl Poller {
    #[constructor]
    fn new(#[endpoint_provider(= 5)] endpoint: Arc<dyn ProvidesEndpoint>) -> Self {}
}
