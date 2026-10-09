use bytes::Bytes;

use margaret_cluster_fixture::margaret::oidc_provider::provider_endpoints::PROVIDER_ENDPOINTS;

use crate::cluster::Cluster;
use crate::cluster_server::ClusterServer;
use crate::endpoint_url::endpoint_url;

/// # Panics
///
/// Panics when the instance does not serve its key set.
pub async fn instance_key_set(cluster: &Cluster, index: usize) -> Bytes {
    cluster
        .client
        .get(endpoint_url(
            &cluster.instance_url(index, ClusterServer::Identity),
            PROVIDER_ENDPOINTS.jwks,
        ))
        .send()
        .await
        .expect("the key set is fetched")
        .error_for_status()
        .expect("the instance serves its key set")
        .bytes()
        .await
        .expect("the key set is read")
}
