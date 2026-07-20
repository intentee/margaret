trait ProvidesEndpoint {}

#[singleton]
struct Poller;

impl Poller {
    #[constructor]
    fn new(#[endpoint_provider] endpoint: Arc<dyn ProvidesEndpoint>) -> Self {}
}
