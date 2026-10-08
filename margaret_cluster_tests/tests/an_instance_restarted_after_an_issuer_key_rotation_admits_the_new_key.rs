use margaret_cluster_tests::bearer_subject::bearer_subject;
use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn an_instance_restarted_after_an_issuer_key_rotation_admits_the_new_key() {
    let mut cluster = Cluster::start(cluster_binary(), 2).await;

    cluster.external_issuer.rotate_external_key();

    assert!(cluster.stop(0).await.success());

    cluster.restart(&[0]).await;

    assert_eq!(
        bearer_subject(
            &cluster,
            cluster.instance_routes(0).public.get_external.url(),
            &cluster.external_issuer.external_token("carol"),
        )
        .await,
        "carol"
    );

    cluster.close().await;
}
