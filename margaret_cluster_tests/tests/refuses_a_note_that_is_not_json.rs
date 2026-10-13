use reqwest::StatusCode;

use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn refuses_a_note_that_is_not_json() {
    let cluster = Cluster::start(cluster_binary(), 1).await;

    assert_eq!(
        cluster
            .client
            .post(cluster.routes().public.post_note.url())
            .body("first draft")
            .send()
            .await
            .expect("the cluster answers")
            .status(),
        StatusCode::UNSUPPORTED_MEDIA_TYPE
    );

    cluster.close().await;
}
