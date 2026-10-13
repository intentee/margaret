use reqwest::StatusCode;
use reqwest::header::COOKIE;

use crate::cluster::Cluster;

/// # Panics
///
/// Panics when the cluster does not answer the profile request.
pub async fn profile_status(cluster: &Cluster, index: usize, session_cookie: &str) -> StatusCode {
    cluster
        .client
        .get(cluster.instance_routes(index).public.get_profile.url())
        .header(COOKIE, session_cookie)
        .send()
        .await
        .expect("the profile request is answered")
        .status()
}
