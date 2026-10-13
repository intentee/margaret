use serde::Deserialize;
use url::Url;

use crate::cluster::Cluster;
use crate::provider_metadata_url::provider_metadata_url;

#[derive(Deserialize)]
pub struct DiscoveredEndpoints {
    pub introspection_endpoint: String,
    pub revocation_endpoint: String,
    pub userinfo_endpoint: String,
}

impl DiscoveredEndpoints {
    /// # Panics
    ///
    /// Panics when the instance does not publish its provider metadata.
    pub async fn discover(cluster: &Cluster, identity: &Url) -> Self {
        cluster
            .client
            .get(provider_metadata_url(identity))
            .send()
            .await
            .expect("the provider metadata is fetched")
            .json()
            .await
            .expect("the provider metadata lists its endpoints")
    }
}
