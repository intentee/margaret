use margaret::framework::active_record::model::Model;
use margaret::framework::authorization_grants::authorization_code_record::AuthorizationCodeRecord;
use margaret_cluster_tests::cluster::Cluster;

pub async fn expire_authorization_codes(cluster: &Cluster) {
    AuthorizationCodeRecord::query()
        .expires_at
        .above(0)
        .update(cluster.database.database.as_ref(), |columns| {
            columns.expires_at.to(0)
        })
        .await
        .expect("the authorization codes expire");
}
