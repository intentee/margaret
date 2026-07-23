use std::sync::Arc;

use reqwest::Client;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;

pub struct JwksClientBundleParams {
    pub endpoint_provider: Arc<dyn ProvidesEndpoint>,
    pub http_client: Client,
}
