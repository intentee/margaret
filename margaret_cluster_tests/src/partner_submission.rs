use reqwest::Response;
use url::Url;

use margaret::framework::oauth_vocabulary::jwt_bearer_client_assertion_type::JWT_BEARER_CLIENT_ASSERTION_TYPE;
use margaret_cluster_fixture::margaret::accepted_clients::clients::auth_accepted_partner_client_accepted_partner_client::accepted_client::ACCEPTED_CLIENT;

use crate::cluster::Cluster;

/// # Panics
///
/// Panics when the submission cannot be sent.
pub async fn partner_submission(cluster: &Cluster, endpoint: Url, token: &str) -> Response {
    cluster
        .client
        .post(endpoint)
        .form(&[
            [
                "client_assertion",
                &cluster.external_issuer.partner_assertion(),
            ],
            ["client_assertion_type", JWT_BEARER_CLIENT_ASSERTION_TYPE],
            ["client_id", ACCEPTED_CLIENT.client_id],
            ["token", token],
        ])
        .send()
        .await
        .expect("the submission is answered")
}
