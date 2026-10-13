use margaret::framework::active_record::model::Model;
use margaret::framework::authorization_grants::pending_authorization_record::PendingAuthorizationRecord;

use crate::cluster::Cluster;

/// # Panics
///
/// Panics when the pending authorizations cannot be expired in the cluster database.
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
