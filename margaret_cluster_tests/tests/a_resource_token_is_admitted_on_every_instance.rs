use reqwest::StatusCode;

use margaret_cluster_fixture::margaret::accepted_clients::clients::auth_accepted_partner_client_accepted_partner_client::accepted_client::ACCEPTED_CLIENT;
use margaret_cluster_tests::bearer_subject::bearer_subject;
use margaret_cluster_tests::client_credentials_grant::client_credentials_grant;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::issued_access_token::IssuedAccessToken;
use margaret_cluster_tests::partner_token::partner_token;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn a_resource_token_is_admitted_on_every_instance() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let response = partner_token(
        &cluster,
        &cluster.instance_url(0, ClusterServer::Identity),
        &client_credentials_grant(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let IssuedAccessToken { access_token } =
        response.json().await.expect("the access token is issued");

    for index in 0..3 {
        assert_eq!(
            bearer_subject(
                &cluster,
                cluster.instance_routes(index).public.get_notes_caller.url(),
                &access_token,
            )
            .await,
            ACCEPTED_CLIENT.client_id
        );
    }

    cluster.close().await;
}
