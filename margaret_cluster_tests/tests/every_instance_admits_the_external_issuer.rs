use margaret_cluster_tests::bearer_subject::bearer_subject;
use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn every_instance_admits_the_external_issuer() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let token = cluster.external_issuer.external_token("bob");

    for index in 0..3 {
        assert_eq!(
            bearer_subject(
                &cluster,
                cluster.instance_routes(index).public.get_external.url(),
                &token,
            )
            .await,
            "bob"
        );
    }

    cluster.close().await;
}
