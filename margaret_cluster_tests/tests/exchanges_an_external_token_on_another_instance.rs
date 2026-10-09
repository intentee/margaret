use reqwest::StatusCode;

use margaret_cluster_fixture::margaret::resource_tokens::notes::AUDIENCE;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::issued_access_token::IssuedAccessToken;
use margaret_cluster_tests::token_exchange::token_exchange;

use crate::assert_verified_by_instance_keys::assert_verified_by_instance_keys;
use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn exchanges_an_external_token_on_another_instance() {
    let cluster = Cluster::start(cluster_binary(), 2).await;
    let response = token_exchange(
        &cluster,
        &cluster.instance_url(1, ClusterServer::Identity),
        &cluster.external_issuer.external_token("alice"),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let IssuedAccessToken { access_token } =
        response.json().await.expect("the access token is issued");

    assert_verified_by_instance_keys(&cluster, 0, &access_token, AUDIENCE).await;

    cluster.close().await;
}
