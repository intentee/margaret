use margaret_cluster_fixture::margaret::resource_tokens::notes::AUDIENCE;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::partner_code::partner_code;
use margaret_cluster_tests::partner_tokens::partner_tokens;

use crate::assert_verified_by_instance_keys::assert_verified_by_instance_keys;
use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn authorizes_consents_and_redeems_on_different_instances() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let code = partner_code(
        &cluster,
        &cluster.instance_url(0, ClusterServer::Identity),
        &cluster.instance_routes(1),
    )
    .await;
    let tokens = partner_tokens(
        &cluster,
        &cluster.instance_url(2, ClusterServer::Identity),
        &code,
    )
    .await;

    for index in 0..3 {
        assert_verified_by_instance_keys(&cluster, index, &tokens.access_token, AUDIENCE).await;
    }

    cluster.close().await;
}
