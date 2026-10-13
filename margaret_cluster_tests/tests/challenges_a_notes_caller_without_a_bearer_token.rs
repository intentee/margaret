use reqwest::StatusCode;

use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn challenges_a_notes_caller_without_a_bearer_token() {
    let cluster = Cluster::start(cluster_binary(), 1).await;

    assert_eq!(
        cluster
            .client
            .get(cluster.routes().public.get_notes_caller.url())
            .send()
            .await
            .expect("the cluster answers")
            .status(),
        StatusCode::UNAUTHORIZED
    );

    cluster.close().await;
}
