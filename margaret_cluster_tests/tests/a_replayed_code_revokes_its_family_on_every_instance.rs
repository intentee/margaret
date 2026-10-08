use reqwest::StatusCode;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::code_grant::code_grant;
use margaret_cluster_tests::partner_code::partner_code;
use margaret_cluster_tests::partner_token::partner_token;
use margaret_cluster_tests::partner_tokens::partner_tokens;
use margaret_cluster_tests::refresh_grant::refresh_grant;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn a_replayed_code_revokes_its_family_on_every_instance() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let identity = |index| cluster.instance_url(index, ClusterServer::Identity);
    let code = partner_code(&cluster, &identity(0), &cluster.instance_routes(0)).await;
    let tokens = partner_tokens(&cluster, &identity(0), &code).await;

    assert_eq!(
        partner_token(&cluster, &identity(1), &code_grant(&code))
            .await
            .status(),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        partner_token(
            &cluster,
            &identity(2),
            &refresh_grant(&tokens.refresh_token)
        )
        .await
        .status(),
        StatusCode::BAD_REQUEST
    );

    cluster.close().await;
}
