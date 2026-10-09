use reqwest::Response;
use url::Url;

use crate::cluster::Cluster;
use crate::discovered_endpoints::DiscoveredEndpoints;
use crate::endpoint_url::endpoint_url;

/// # Panics
///
/// Panics when the userinfo request cannot be sent.
pub async fn userinfo(cluster: &Cluster, identity: &Url, access_token: &str) -> Response {
    let DiscoveredEndpoints {
        userinfo_endpoint, ..
    } = DiscoveredEndpoints::discover(cluster, identity).await;

    cluster
        .client
        .get(endpoint_url(identity, &userinfo_endpoint))
        .bearer_auth(access_token)
        .send()
        .await
        .expect("the userinfo request is answered")
}
