use reqwest::StatusCode;

use margaret_cluster_fixture::margaret::routes::Routes;
use margaret_cluster_fixture::routes::public::session_refresh_token::SessionRefreshToken;

use crate::cluster::Cluster;
use crate::refreshed_tokens::RefreshedTokens;

/// # Panics
///
/// Panics when the instance does not mint tokens for the refresh token.
pub async fn minted_tokens(
    cluster: &Cluster,
    routes: &Routes,
    refresh_token: String,
) -> RefreshedTokens {
    let response = cluster
        .client
        .post(routes.public.post_minted_token.url())
        .json(&SessionRefreshToken { refresh_token })
        .send()
        .await
        .expect("the mint request is answered");

    assert_eq!(response.status(), StatusCode::OK);

    response.json().await.expect("the instance mints tokens")
}
