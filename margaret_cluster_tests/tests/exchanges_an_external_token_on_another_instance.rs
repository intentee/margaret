use reqwest::StatusCode;

use margaret::framework::jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret::framework::jwt_verification::jwt_verification::JwtVerification;
use margaret_cluster_fixture::margaret::resource_tokens::notes::audience::AUDIENCE;
use margaret_cluster_tests::access_token_verification::access_token_verification;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::instance_key_set_parsing::instance_key_set_parsing;
use margaret_cluster_tests::issued_access_token::IssuedAccessToken;
use margaret_cluster_tests::token_exchange::token_exchange;

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

    let KeySetDocumentParsing::Accepted(document) = instance_key_set_parsing(&cluster, 0).await
    else {
        panic!("the instance publishes a key set");
    };

    assert!(matches!(
        access_token_verification(&document, &access_token, AUDIENCE),
        JwtVerification::Verified(_)
    ));

    cluster.close().await;
}
