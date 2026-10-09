use reqwest::StatusCode;
use reqwest::header::COOKIE;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::response_cookie::response_cookie;
use margaret_cluster_tests::session_access_cookie::SESSION_ACCESS_COOKIE;
use margaret_cluster_tests::started_alice_session::started_alice_session;

use crate::cluster_binary::cluster_binary;
use crate::profile_status::profile_status;

#[tokio::test]
async fn a_session_started_on_one_instance_refreshes_on_another() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let alice = started_alice_session(&cluster, &cluster.instance_routes(1)).await;
    let refreshed = cluster
        .client
        .get(cluster.instance_routes(2).public.get_profile.url())
        .header(COOKIE, &alice.secret)
        .send()
        .await
        .expect("the profile request is answered");

    assert_eq!(refreshed.status(), StatusCode::OK);
    assert_eq!(
        profile_status(
            &cluster,
            0,
            &response_cookie(&refreshed, SESSION_ACCESS_COOKIE)
        )
        .await,
        StatusCode::OK
    );

    cluster.close().await;
}
