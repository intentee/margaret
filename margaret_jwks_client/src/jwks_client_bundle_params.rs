use std::sync::Arc;

use rustls::ClientConfig;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

pub struct JwksClientBundleParams {
    pub client_config: ClientConfig,
    pub endpoint: Arc<dyn ProvidesEndpoint>,
}
