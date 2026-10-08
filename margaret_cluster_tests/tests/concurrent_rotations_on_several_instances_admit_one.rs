use futures_util::future::join_all;
use reqwest::StatusCode;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::partner_code::partner_code;
use margaret_cluster_tests::partner_token::partner_token;
use margaret_cluster_tests::partner_tokens::partner_tokens;
use margaret_cluster_tests::refresh_grant::refresh_grant;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn concurrent_rotations_on_several_instances_admit_one() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let identities: Vec<_> = (0..3)
        .map(|index| cluster.instance_url(index, ClusterServer::Identity))
        .collect();
    let code = partner_code(&cluster, &identities[0], &cluster.instance_routes(0)).await;
    let tokens = partner_tokens(&cluster, &identities[0], &code).await;
    let grant = refresh_grant(&tokens.refresh_token);
    let statuses: Vec<StatusCode> = join_all(
        identities
            .iter()
            .map(|identity| partner_token(&cluster, identity, &grant)),
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

    cluster.close().await;
}
