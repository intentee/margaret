use reqwest::StatusCode;

use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn a_terminated_instance_exits_cleanly_while_peers_serve() {
    let mut cluster = Cluster::start(cluster_binary(), 2).await;

    assert!(cluster.stop(0).await.success());

    for _ in 0..2 {
        assert_eq!(
            cluster
                .client
                .get(cluster.routes().public.get_health.url())
                .send()
                .await
                .expect("the front door answers")
                .status(),
            StatusCode::OK
        );
    }

    cluster.close().await;
}
