use futures_util::future::join_all;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::instance_key_set::instance_key_set;

use crate::cluster_binary::cluster_binary;
use crate::signing_key_generations::signing_key_generations;

#[tokio::test]
async fn instances_started_together_on_an_empty_database_create_one_key_set() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let documents = join_all((0..3).map(|index| instance_key_set(&cluster, index))).await;

    assert_eq!(signing_key_generations(&cluster).await, vec![1]);
    assert_eq!(documents[1], documents[0]);
    assert_eq!(documents[2], documents[0]);

    cluster.close().await;
}
