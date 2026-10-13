use reqwest::StatusCode;

use margaret_cluster_tests::authorized_sign_in::authorized_sign_in;
use margaret_cluster_tests::begun_sign_in::BegunSignIn;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::completed_sign_in::completed_sign_in;
use margaret_cluster_tests::started_alice_session::started_alice_session;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn a_replayed_sign_in_callback_is_refused_on_another_instance() {
    let cluster = Cluster::start(cluster_binary(), 2).await;
    let alice = started_alice_session(&cluster, &cluster.instance_routes(0)).await;
    let begun = BegunSignIn::begin(&cluster, &cluster.instance_routes(0)).await;
    let callback = authorized_sign_in(&cluster, &begun, &cluster.instance_routes(0), &alice).await;
    let complete_on = |index| cluster.instance_url(index, ClusterServer::Public);

    assert_eq!(
        completed_sign_in(&cluster, &begun, &callback, &complete_on(0))
            .await
            .status(),
        StatusCode::SEE_OTHER
    );
    assert_eq!(
        completed_sign_in(&cluster, &begun, &callback, &complete_on(1))
            .await
            .status(),
        StatusCode::FORBIDDEN
    );

    cluster.close().await;
}
