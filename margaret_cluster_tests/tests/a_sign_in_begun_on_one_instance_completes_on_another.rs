use reqwest::StatusCode;

use margaret_cluster_tests::authorized_sign_in::authorized_sign_in;
use margaret_cluster_tests::begun_sign_in::BegunSignIn;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::completed_sign_in::completed_sign_in;
use margaret_cluster_tests::response_cookies::response_cookies;
use margaret_cluster_tests::started_alice_session::started_alice_session;

use crate::cluster_binary::cluster_binary;
use crate::profile_status::profile_status;

#[tokio::test]
async fn a_sign_in_begun_on_one_instance_completes_on_another() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let alice = started_alice_session(&cluster, &cluster.instance_routes(0)).await;
    let begun = BegunSignIn::begin(&cluster, &cluster.instance_routes(0)).await;
    let callback = authorized_sign_in(&cluster, &begun, &cluster.instance_routes(1), &alice).await;
    let completed = completed_sign_in(
        &cluster,
        &begun,
        &callback,
        &cluster.instance_url(2, ClusterServer::Public),
    )
    .await;

    assert_eq!(completed.status(), StatusCode::SEE_OTHER);

    let session_cookie = response_cookies(&completed);

    for index in 0..3 {
        assert_eq!(
            profile_status(&cluster, index, &session_cookie).await,
            StatusCode::OK
        );
    }

    cluster.close().await;
}
