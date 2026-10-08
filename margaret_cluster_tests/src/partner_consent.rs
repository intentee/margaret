use reqwest::StatusCode;
use reqwest::header::COOKIE;
use url::Url;
use uuid::Uuid;

use margaret::framework::oauth_vocabulary::code_challenge_method::CodeChallengeMethod;
use margaret_cluster_fixture::margaret::accepted_clients::clients::auth_accepted_partner_client_accepted_partner_client::accepted_client::ACCEPTED_CLIENT;
use margaret_cluster_fixture::margaret::oidc_provider::provider_endpoints::authorization_endpoint_url::AUTHORIZATION_ENDPOINT_URL;
use margaret_oidc_provider_tests::pkce_challenge::pkce_challenge;

use crate::alice_session_cookie::alice_session_cookie;
use crate::cluster::Cluster;
use crate::consent_required::ConsentRequired;
use crate::endpoint_url::endpoint_url;
use crate::partner_redirect_uri::PARTNER_REDIRECT_URI;

/// # Panics
///
/// Panics when the identity server does not ask Alice for her consent.
pub async fn partner_consent(cluster: &Cluster, identity: &Url) -> Uuid {
    let response = cluster
        .client
        .get(endpoint_url(identity, AUTHORIZATION_ENDPOINT_URL))
        .header(COOKIE, alice_session_cookie())
        .query(&[
            ["client_id", ACCEPTED_CLIENT.client_id],
            ["code_challenge", &pkce_challenge()],
            [
                "code_challenge_method",
                CodeChallengeMethod::S256.wire_name(),
            ],
            ["nonce", "cluster-nonce"],
            ["redirect_uri", PARTNER_REDIRECT_URI],
            ["response_type", "code"],
            ["scope", "openid profile"],
            ["state", "cluster-state"],
        ])
        .send()
        .await
        .expect("the authorization request is answered");

    assert_eq!(response.status(), StatusCode::OK);

    response
        .json::<ConsentRequired>()
        .await
        .expect("the identity server asks for consent")
        .consent
}
