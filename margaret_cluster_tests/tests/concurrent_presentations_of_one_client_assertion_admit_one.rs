use futures_util::future::join_all;
use reqwest::StatusCode;

use margaret_cluster_tests::client_credentials_grant::client_credentials_grant;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::partner_token_asserted::partner_token_asserted;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn concurrent_presentations_of_one_client_assertion_admit_one() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let assertion = cluster.external_issuer.partner_assertion();
    let identities: Vec<_> = (0..3)
        .map(|index| cluster.instance_url(index, ClusterServer::Identity))
        .collect();
    let grant = client_credentials_grant();
    let statuses: Vec<StatusCode> = join_all(
        identities
            .iter()
            .map(|identity| partner_token_asserted(&cluster, identity, &grant, &assertion)),
    )
    .await
    .iter()
    .map(reqwest::Response::status)
    .collect();

    assert_eq!(
        statuses
            .iter()
            .filter(|status| **status == StatusCode::OK)
            .count(),
        1
    );
    assert_eq!(
        statuses
            .iter()
            .filter(|status| **status == StatusCode::UNAUTHORIZED)
            .count(),
        2
    );

    cluster.close().await;
}
