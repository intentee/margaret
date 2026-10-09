use reqwest::StatusCode;

use margaret_cluster_tests::authorized_sign_in::authorized_sign_in;
use margaret_cluster_tests::begun_sign_in::BegunSignIn;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::completed_sign_in::completed_sign_in;
use margaret_cluster_tests::started_alice_session::started_alice_session;

use crate::cluster_binary::cluster_binary;
use crate::overdue_signing_keys::overdue_signing_keys;
use crate::signing_key_generations::signing_key_generations;

#[tokio::test]
async fn a_sign_in_transaction_crosses_a_key_roll() {
    let mut cluster = Cluster::start(cluster_binary(), 2).await;
    let alice = started_alice_session(&cluster, &cluster.instance_routes(0)).await;
    let begun = BegunSignIn::begin(&cluster, &cluster.instance_routes(0)).await;

    assert!(cluster.stop(1).await.success());

    overdue_signing_keys(&cluster).await;
    cluster.restart(&[1]).await;

    assert_eq!(signing_key_generations(&cluster).await, vec![2]);

    let callback = authorized_sign_in(&cluster, &begun, &cluster.instance_routes(1), &alice).await;

    assert_eq!(
        completed_sign_in(
            &cluster,
            &begun,
            &callback,
            &cluster.instance_url(1, ClusterServer::Public),
        )
        .await
        .status(),
        StatusCode::SEE_OTHER
    );

    cluster.close().await;
}
