use reqwest::StatusCode;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::consent_decision::consent_decision;
use margaret_cluster_tests::partner_consent::partner_consent;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn a_decided_consent_is_unknown_on_another_instance() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let consent =
        partner_consent(&cluster, &cluster.instance_url(0, ClusterServer::Identity)).await;

    assert_eq!(
        consent_decision(&cluster, &cluster.instance_routes(1), consent)
            .await
            .status(),
        StatusCode::SEE_OTHER
    );
    assert_eq!(
        consent_decision(&cluster, &cluster.instance_routes(2), consent)
            .await
            .status(),
        StatusCode::NOT_FOUND
    );

    cluster.close().await;
}
