use margaret::framework::active_record::model::Model;
use margaret::framework::authorization_grants::pending_authorization_record::PendingAuthorizationRecord;
use margaret_cluster_tests::cluster::Cluster;

pub async fn expire_pending_authorizations(cluster: &Cluster) {
    PendingAuthorizationRecord::query()
        .expires_at
        .above(0)
        .update(cluster.database.database.as_ref(), |columns| {
            columns.expires_at.to(0)
        })
        .await
        .expect("the pending authorizations expire");
}
