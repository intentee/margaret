use futures_util::future::join_all;

use margaret::framework::model::qualified_framework_table::qualified_framework_table;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::instance_key_set::instance_key_set;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn instances_started_together_on_an_empty_database_create_one_key_set() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let generations: Vec<i64> = cluster
        .database
        .database
        .client()
        .await
        .expect("a connection is checked out")
        .query(
            &format!(
                "SELECT generation FROM {}",
                qualified_framework_table("signing_key_sets")
            ),
            &[],
        )
        .await
        .expect("the key sets are read")
        .iter()
        .map(|row| row.get("generation"))
        .collect();
    let documents = join_all((0..3).map(|index| instance_key_set(&cluster, index))).await;

    assert_eq!(generations, vec![1]);
    assert_eq!(documents[1], documents[0]);
    assert_eq!(documents[2], documents[0]);

    cluster.close().await;
}
