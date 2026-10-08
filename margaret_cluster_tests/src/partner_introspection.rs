use url::Url;

use crate::cluster::Cluster;
use crate::discovered_endpoints::DiscoveredEndpoints;
use crate::endpoint_url::endpoint_url;
use crate::introspection::Introspection;
use crate::partner_submission::partner_submission;

/// # Panics
///
/// Panics when the token is not introspected.
pub async fn partner_introspection(
    cluster: &Cluster,
    identity: &Url,
    token: &str,
) -> Introspection {
    let DiscoveredEndpoints {
        introspection_endpoint,
        ..
    } = DiscoveredEndpoints::discover(cluster, identity).await;

    partner_submission(
        cluster,
        endpoint_url(identity, &introspection_endpoint),
        token,
    )
    .await
    .json()
    .await
    .expect("the introspection answers")
}
