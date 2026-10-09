use margaret_cluster_tests::cluster::Cluster;

use crate::signing_key_sets::signing_key_sets;

pub async fn signing_key_generations(cluster: &Cluster) -> Vec<i64> {
    signing_key_sets(cluster)
        .await
        .into_iter()
        .map(|key_set| key_set.generation)
        .collect()
}
