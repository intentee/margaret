use reqwest::StatusCode;

use margaret_cluster_tests::client_credentials_grant::client_credentials_grant;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::partner_token_asserted::partner_token_asserted;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn a_restarted_instance_remembers_client_assertions() {
    let mut cluster = Cluster::start(cluster_binary(), 1).await;
    let assertion = cluster.external_issuer.partner_assertion();
    let identity = cluster.instance_url(0, ClusterServer::Identity);

    assert_eq!(
        partner_token_asserted(&cluster, &identity, &client_credentials_grant(), &assertion)
            .await
            .status(),
        StatusCode::OK
    );

    cluster.crash(0).await;
    cluster.restart(&[0]).await;

    assert_eq!(
        partner_token_asserted(&cluster, &identity, &client_credentials_grant(), &assertion)
            .await
            .status(),
        StatusCode::UNAUTHORIZED
    );

    cluster.close().await;
}
