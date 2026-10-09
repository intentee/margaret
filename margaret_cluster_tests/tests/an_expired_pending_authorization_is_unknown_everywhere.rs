use reqwest::StatusCode;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::consent_decision::consent_decision;
use margaret_cluster_tests::partner_consent::partner_consent;

use crate::cluster_binary::cluster_binary;
use crate::expire_pending_authorizations::expire_pending_authorizations;

#[tokio::test]
async fn an_expired_pending_authorization_is_unknown_everywhere() {
    let cluster = Cluster::start(cluster_binary(), 2).await;
    let consent =
        partner_consent(&cluster, &cluster.instance_url(0, ClusterServer::Identity)).await;

    expire_pending_authorizations(&cluster).await;

    assert_eq!(
        consent_decision(&cluster, &cluster.instance_routes(1), consent)
            .await
            .status(),
        StatusCode::NOT_FOUND
    );

    cluster.close().await;
}
