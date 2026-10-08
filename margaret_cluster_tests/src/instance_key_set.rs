use bytes::Bytes;

use margaret_cluster_fixture::margaret::oidc_provider::provider_endpoints::PROVIDER_ENDPOINTS;

use crate::cluster::Cluster;
use crate::cluster_server::ClusterServer;
use crate::declared_path::declared_path;

/// # Panics
///
/// Panics when the instance does not serve its key set.
pub async fn instance_key_set(cluster: &Cluster, index: usize) -> Bytes {
    cluster
        .client
        .get(
            cluster
                .instance_url(index, ClusterServer::Identity)
                .join(&declared_path(PROVIDER_ENDPOINTS.jwks))
                .expect("the key set URL joins"),
        )
        .send()
        .await
        .expect("the key set is fetched")
        .error_for_status()
        .expect("the instance serves its key set")
        .bytes()
        .await
        .expect("the key set is read")
}
