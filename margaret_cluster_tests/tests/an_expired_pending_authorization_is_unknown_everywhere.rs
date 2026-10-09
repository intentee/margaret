use reqwest::StatusCode;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::consent_decision::consent_decision;
use margaret_cluster_tests::partner_consent::partner_consent;
use margaret_cluster_tests::started_alice_session::started_alice_session;

use crate::cluster_binary::cluster_binary;
use crate::expire_pending_authorizations::expire_pending_authorizations;

#[tokio::test]
async fn an_expired_pending_authorization_is_unknown_everywhere() {
    let cluster = Cluster::start(cluster_binary(), 2).await;
    let alice = started_alice_session(&cluster, &cluster.instance_routes(0)).await;
    let consent = partner_consent(
        &cluster,
        &cluster.instance_url(0, ClusterServer::Identity),
        &alice.cookies,
    )
    .await;

    expire_pending_authorizations(&cluster).await;

    assert_eq!(
        consent_decision(
            &cluster,
            &cluster.instance_routes(1),
            consent,
            &alice.cookies
        )
        .await
        .status(),
        StatusCode::NOT_FOUND
    );

    cluster.close().await;
}
