use reqwest::StatusCode;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::started_alice_session::started_alice_session;

use crate::cluster_binary::cluster_binary;
use crate::profile_status::profile_status;

#[tokio::test]
async fn a_crashed_instance_restarted_admits_earlier_tokens() {
    let mut cluster = Cluster::start(cluster_binary(), 2).await;
    let alice = started_alice_session(&cluster, &cluster.instance_routes(0)).await;

    cluster.crash(0).await;
    cluster.restart(&[0]).await;

    assert_eq!(
        profile_status(&cluster, 0, &alice.access).await,
        StatusCode::OK
    );

    cluster.close().await;
}
