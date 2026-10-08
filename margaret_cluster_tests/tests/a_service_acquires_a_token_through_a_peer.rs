use margaret_cluster_fixture::models::token_acquisition_outcome::TokenAcquisitionOutcome;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::instance_admission::InstanceAdmission;

use crate::cluster_binary::cluster_binary;
use crate::poll_until::poll_until;

async fn attempts(cluster: &Cluster) -> i64 {
    cluster
        .database
        .database
        .client()
        .await
        .expect("a connection is checked out")
        .query_one("SELECT count(*) AS attempts FROM token_acquisitions", &[])
        .await
        .expect("the attempts are counted")
        .get("attempts")
}

async fn acquired_tokens(cluster: &Cluster) -> i64 {
    cluster
        .database
        .database
        .client()
        .await
        .expect("a connection is checked out")
        .query_one(
            "SELECT count(*) AS acquired FROM token_acquisitions WHERE outcome = $1",
            &[&TokenAcquisitionOutcome::Acquired.stored()],
        )
        .await
        .expect("the acquisitions are counted")
        .get("acquired")
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
