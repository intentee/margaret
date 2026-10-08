use margaret::framework::model::qualified_framework_table::qualified_framework_table;
use margaret_cluster_tests::cluster::Cluster;

pub async fn signing_key_generations(cluster: &Cluster) -> Vec<i64> {
    cluster
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
        .collect()
}
