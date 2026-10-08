use reqwest::StatusCode;
use reqwest::header::COOKIE;

use margaret_cluster_fixture::margaret::routes::Routes;
use margaret_cluster_fixture::routes::public::session_refresh_token::SessionRefreshToken;

use crate::cluster::Cluster;

/// # Panics
///
/// Panics when the instance does not issue a refresh token for the session.
pub async fn session_refresh_token(
    cluster: &Cluster,
    routes: &Routes,
    session_cookie: &str,
) -> String {
    let response = cluster
        .client
        .post(routes.public.post_session_token.url())
        .header(COOKIE, session_cookie)
        .send()
        .await
        .expect("the session token request is answered");

    assert_eq!(response.status(), StatusCode::OK);

    response
        .json::<SessionRefreshToken>()
        .await
        .expect("the instance issues a refresh token")
        .refresh_token
}
