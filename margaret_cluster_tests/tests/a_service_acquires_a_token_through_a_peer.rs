use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::page::Page;
use margaret_cluster_fixture::models::token_acquisition::TokenAcquisition;
use margaret_cluster_fixture::models::token_acquisition_outcome::TokenAcquisitionOutcome;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::instance_admission::InstanceAdmission;

use crate::cluster_binary::cluster_binary;
use crate::poll_until::poll_until;

async fn attempts(cluster: &Cluster) -> usize {
    let Page { records, .. } = TokenAcquisition::query()
        .id
        .ascending()
        .limit::<3>()
        .fetch(cluster.database.database.as_ref())
        .await
        .expect("the attempts are read");

    records.len()
}

async fn acquired_tokens(cluster: &Cluster) -> usize {
    let Page { records, .. } = TokenAcquisition::query()
        .outcome
        .eq(TokenAcquisitionOutcome::Acquired)
        .id
        .ascending()
        .limit::<3>()
        .fetch(cluster.database.database.as_ref())
        .await
        .expect("the acquisitions are read");

    records.len()
}

#[tokio::test]
async fn a_service_acquires_a_token_through_a_peer() {
    let mut cluster = Cluster::start(cluster_binary(), 1).await;

    poll_until(|| async { attempts(&cluster).await == 1 }).await;

    let acquired_before = acquired_tokens(&cluster).await;
    let unadmitted = cluster.add_member().await;

    cluster
        .launch(&[unadmitted], InstanceAdmission::Withheld)
        .await;

    poll_until(|| async { attempts(&cluster).await == 2 }).await;

    assert_eq!(acquired_tokens(&cluster).await, acquired_before + 1);

    cluster.close().await;
}
