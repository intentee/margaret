use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::page::Page;
use margaret::framework::signing_keys::signing_key_set::SigningKeySet;
use margaret_cluster_tests::cluster::Cluster;

pub async fn signing_key_sets(cluster: &Cluster) -> Vec<SigningKeySet> {
    let Page { records, .. } = SigningKeySet::query()
        .name
        .ascending()
        .limit::<2>()
        .fetch(cluster.database.database.as_ref())
        .await
        .expect("the key sets are read");

    records
}
