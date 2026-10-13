use reqwest::Response;
use url::Url;

use crate::cluster::Cluster;
use crate::discovered_endpoints::DiscoveredEndpoints;
use crate::endpoint_url::endpoint_url;
use crate::partner_submission::partner_submission;

pub async fn partner_revocation(cluster: &Cluster, identity: &Url, token: &str) -> Response {
    let DiscoveredEndpoints {
        revocation_endpoint,
        ..
    } = DiscoveredEndpoints::discover(cluster, identity).await;

    partner_submission(cluster, endpoint_url(identity, &revocation_endpoint), token).await
}
