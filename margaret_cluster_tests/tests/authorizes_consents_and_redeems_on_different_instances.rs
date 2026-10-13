use margaret::framework::jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret::framework::jwt_verification::jwt_verification::JwtVerification;
use margaret_cluster_fixture::margaret::resource_tokens::notes::audience::AUDIENCE;
use margaret_cluster_tests::access_token_verification::access_token_verification;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::instance_key_set_parsing::instance_key_set_parsing;
use margaret_cluster_tests::partner_code::partner_code;
use margaret_cluster_tests::partner_tokens::partner_tokens;
use margaret_cluster_tests::started_alice_session::started_alice_session;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn authorizes_consents_and_redeems_on_different_instances() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let alice = started_alice_session(&cluster, &cluster.instance_routes(0)).await;
    let code = partner_code(
        &cluster,
        &cluster.instance_url(0, ClusterServer::Identity),
        &cluster.instance_routes(1),
        &alice.cookies,
    )
    .await;
    let tokens = partner_tokens(
        &cluster,
        &cluster.instance_url(2, ClusterServer::Identity),
        &code,
    )
    .await;

    for index in 0..3 {
        let KeySetDocumentParsing::Accepted(document) =
            instance_key_set_parsing(&cluster, index).await
        else {
            panic!("the instance publishes a key set");
        };

        assert!(matches!(
            access_token_verification(&document, &tokens.access_token, AUDIENCE),
            JwtVerification::Verified(_)
        ));
    }

    cluster.close().await;
}
