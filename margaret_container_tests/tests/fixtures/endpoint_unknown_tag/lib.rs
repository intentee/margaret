use std::sync::Arc;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

#[singleton]
struct Client;

impl Client {
    #[constructor]
    fn new(#[endpoint_provider(missing)] issuer: Arc<dyn ProvidesEndpoint>) -> Self {}
}
