use reqwest::StatusCode;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::code_grant::code_grant;
use margaret_cluster_tests::partner_code::partner_code;
use margaret_cluster_tests::partner_token::partner_token;
use margaret_cluster_tests::started_alice_session::started_alice_session;

use crate::cluster_binary::cluster_binary;
use crate::expire_authorization_codes::expire_authorization_codes;

#[tokio::test]
async fn an_expired_code_is_refused_everywhere() {
    let cluster = Cluster::start(cluster_binary(), 2).await;
    let alice = started_alice_session(&cluster, &cluster.instance_routes(0)).await;
    let code = partner_code(
        &cluster,
        &cluster.instance_url(0, ClusterServer::Identity),
        &cluster.instance_routes(0),
        &alice.cookies,
    )
    .await;

    expire_authorization_codes(&cluster).await;

    assert_eq!(
        partner_token(
            &cluster,
            &cluster.instance_url(1, ClusterServer::Identity),
            &code_grant(&code),
        )
        .await
        .status(),
        StatusCode::BAD_REQUEST
    );

    cluster.close().await;
}
