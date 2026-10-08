use reqwest::StatusCode;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::partner_code::partner_code;
use margaret_cluster_tests::partner_revocation::partner_revocation;
use margaret_cluster_tests::partner_token::partner_token;
use margaret_cluster_tests::partner_tokens::partner_tokens;
use margaret_cluster_tests::refresh_grant::refresh_grant;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn a_revocation_on_one_instance_refuses_the_refresh_token_everywhere() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let identity = |index| cluster.instance_url(index, ClusterServer::Identity);
    let code = partner_code(&cluster, &identity(0), &cluster.instance_routes(0)).await;
    let tokens = partner_tokens(&cluster, &identity(0), &code).await;

    assert_eq!(
        partner_revocation(&cluster, &identity(0), &tokens.refresh_token)
            .await
            .status(),
        StatusCode::OK
    );

    for index in 1..3 {
        assert_eq!(
            partner_token(
                &cluster,
                &identity(index),
                &refresh_grant(&tokens.refresh_token)
            )
            .await
            .status(),
            StatusCode::BAD_REQUEST
        );
    }

    cluster.close().await;
}
