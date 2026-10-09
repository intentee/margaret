use futures_util::future::join_all;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::provider_metadata_url::provider_metadata_url;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn discovery_is_identical_everywhere() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let documents = join_all((0..3).map(|index| {
        cluster
            .client
            .get(provider_metadata_url(
                &cluster.instance_url(index, ClusterServer::Identity),
            ))
            .send()
    }))
    .await;
    let mut bodies = Vec::new();

    for document in documents {
        bodies.push(
            document
                .expect("the provider metadata is fetched")
                .bytes()
                .await
                .expect("the provider metadata is read"),
        );
    }

    assert_eq!(bodies[1], bodies[0]);
    assert_eq!(bodies[2], bodies[0]);

    cluster.close().await;
}
