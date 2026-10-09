use reqwest::Response;
use url::Url;

use margaret::framework::oauth_vocabulary::grant_type::GrantType;
use margaret::framework::oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_cluster_fixture::margaret::accepted_clients::clients::auth_accepted_exchange_client_accepted_exchange_client::accepted_client::ACCEPTED_CLIENT;
use margaret_cluster_fixture::margaret::oidc_provider::provider_endpoints::PROVIDER_ENDPOINTS;

use crate::cluster::Cluster;
use crate::endpoint_url::endpoint_url;

/// # Panics
///
/// Panics when the exchange request cannot be sent.
pub async fn token_exchange(cluster: &Cluster, identity: &Url, subject_token: &str) -> Response {
    cluster
        .client
        .post(endpoint_url(identity, PROVIDER_ENDPOINTS.token))
        .form(&[
            ["client_id", ACCEPTED_CLIENT.client_id],
            ["grant_type", GrantType::TokenExchange.wire_name()],
            ["subject_token", subject_token],
            ["subject_token_type", SubjectTokenType::Jwt.urn()],
        ])
        .send()
        .await
        .expect("the exchange request is answered")
}
