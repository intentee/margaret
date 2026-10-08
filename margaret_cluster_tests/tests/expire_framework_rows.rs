use margaret::framework::model::qualified_framework_table::qualified_framework_table;
use margaret_cluster_tests::cluster::Cluster;

pub async fn expire_framework_rows(cluster: &Cluster, table: &str) {
    cluster
        .database
        .database
        .client()
        .await
        .expect("a connection is checked out")
        .execute(
            &format!(
                "UPDATE {} SET expires_at = 0",
                qualified_framework_table(table)
            ),
            &[],
        )
        .await
        .expect("the rows expire");
}
