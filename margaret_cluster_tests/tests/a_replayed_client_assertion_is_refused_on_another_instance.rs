use reqwest::StatusCode;

use margaret_cluster_tests::client_credentials_grant::client_credentials_grant;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::partner_token_asserted::partner_token_asserted;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn a_replayed_client_assertion_is_refused_on_another_instance() {
    let cluster = Cluster::start(cluster_binary(), 2).await;
    let assertion = cluster.external_issuer.partner_assertion();
    let presented = async |index| {
        partner_token_asserted(
            &cluster,
            &cluster.instance_url(index, ClusterServer::Identity),
            &client_credentials_grant(),
            &assertion,
        )
        .await
        .status()
    };

    assert_eq!(presented(0).await, StatusCode::OK);
    assert_eq!(presented(1).await, StatusCode::UNAUTHORIZED);

    cluster.close().await;
}
