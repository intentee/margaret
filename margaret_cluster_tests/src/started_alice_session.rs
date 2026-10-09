use reqwest::StatusCode;

use margaret_cluster_fixture::margaret::routes::Routes;

use crate::alice_session::AliceSession;
use crate::cluster::Cluster;
use crate::response_cookie::response_cookie;
use crate::session_access_cookie::SESSION_ACCESS_COOKIE;
use crate::session_secret_cookie::SESSION_SECRET_COOKIE;

/// # Panics
///
/// Panics when the instance does not start a session of Alice.
pub async fn started_alice_session(cluster: &Cluster, routes: &Routes) -> AliceSession {
    let response = cluster
        .client
        .post(routes.public.post_alice_session.url())
        .send()
        .await
        .expect("the session of Alice is started");

    assert_eq!(response.status(), StatusCode::OK);

    let access = response_cookie(&response, SESSION_ACCESS_COOKIE);
    let secret = response_cookie(&response, SESSION_SECRET_COOKIE);

    AliceSession {
        cookies: format!("{secret}; {access}"),
        access,
        secret,
    }
}
