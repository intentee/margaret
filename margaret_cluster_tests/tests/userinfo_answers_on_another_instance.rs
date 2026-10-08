use reqwest::StatusCode;
use serde::Deserialize;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::partner_code::partner_code;
use margaret_cluster_tests::partner_tokens::partner_tokens;
use margaret_cluster_tests::userinfo::userinfo;

use crate::cluster_binary::cluster_binary;

#[derive(Deserialize)]
struct ProfileClaims {
    name: String,
}

#[tokio::test]
async fn userinfo_answers_on_another_instance() {
    let cluster = Cluster::start(cluster_binary(), 2).await;
    let identity = |index| cluster.instance_url(index, ClusterServer::Identity);
    let code = partner_code(&cluster, &identity(0), &cluster.instance_routes(0)).await;
    let tokens = partner_tokens(&cluster, &identity(0), &code).await;
    let response = userinfo(&cluster, &identity(1), &tokens.access_token).await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .json::<ProfileClaims>()
            .await
            .expect("the userinfo carries the profile")
            .name,
        "alice"
    );

    cluster.close().await;
}
