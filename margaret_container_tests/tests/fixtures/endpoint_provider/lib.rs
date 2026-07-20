trait ProvidesEndpoint {}

#[singleton]
#[provides_endpoint(jwks)]
struct InternalEndpoint;

impl InternalEndpoint {
    #[constructor]
    fn new() -> Self {}
}

#[singleton]
struct Poller;

impl Poller {
    #[constructor]
    fn new(#[endpoint_provider(jwks)] endpoint: Arc<dyn ProvidesEndpoint>) -> Self {}
}
