use reqwest::Response;
use url::Url;

use margaret::framework::oauth_vocabulary::jwt_bearer_client_assertion_type::JWT_BEARER_CLIENT_ASSERTION_TYPE;
use margaret_cluster_fixture::margaret::accepted_clients::clients::auth_accepted_partner_client_accepted_partner_client::accepted_client::ACCEPTED_CLIENT;
use margaret_cluster_fixture::margaret::oidc_provider::provider_endpoints::PROVIDER_ENDPOINTS;

use crate::cluster::Cluster;
use crate::endpoint_url::endpoint_url;

/// # Panics
///
/// Panics when the token request cannot be sent.
pub async fn partner_token_asserted(
    cluster: &Cluster,
    identity: &Url,
    grant: &[[&str; 2]],
    assertion: &str,
) -> Response {
    let authentication = [
        ["client_assertion", assertion],
        ["client_assertion_type", JWT_BEARER_CLIENT_ASSERTION_TYPE],
        ["client_id", ACCEPTED_CLIENT.client_id],
    ];

    cluster
        .client
        .post(endpoint_url(identity, PROVIDER_ENDPOINTS.token))
        .form(&[grant, authentication.as_slice()].concat())
        .send()
        .await
        .expect("the token request is answered")
}
