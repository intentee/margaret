use reqwest::Response;
use reqwest::header::COOKIE;
use uuid::Uuid;

use margaret_cluster_fixture::margaret::routes::Routes;

use crate::alice_session_cookie::alice_session_cookie;
use crate::cluster::Cluster;

/// # Panics
///
/// Panics when the decision cannot be sent.
pub async fn consent_decision(cluster: &Cluster, routes: &Routes, consent: Uuid) -> Response {
    cluster
        .client
        .post(routes.identity.post_authorization_consent.url())
        .header(COOKIE, alice_session_cookie())
        .form(&[["decision", "approve"], ["id", &consent.to_string()]])
        .send()
        .await
        .expect("the consent decision is answered")
}
