use reqwest::StatusCode;

use margaret_cluster_tests::client_credentials_grant::client_credentials_grant;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::issued_access_token::IssuedAccessToken;
use margaret_cluster_tests::partner_introspection::partner_introspection;
use margaret_cluster_tests::partner_token::partner_token;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn client_credentials_introspect_active_on_another_instance() {
    let cluster = Cluster::start(cluster_binary(), 2).await;
    let identity = |index| cluster.instance_url(index, ClusterServer::Identity);
    let response = partner_token(&cluster, &identity(0), &client_credentials_grant()).await;

    assert_eq!(response.status(), StatusCode::OK);

    let IssuedAccessToken { access_token } =
        response.json().await.expect("the access token is issued");

    assert!(
        partner_introspection(&cluster, &identity(1), &access_token)
            .await
            .active
    );

    cluster.close().await;
}
