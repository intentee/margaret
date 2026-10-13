use reqwest::StatusCode;
use serde_json::json;
use uuid::Uuid;

use margaret_cluster_tests::addressed_route::addressed_route;
use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn refuses_patching_a_note_that_does_not_exist() {
    let cluster = Cluster::start(cluster_binary(), 1).await;

    assert_eq!(
        cluster
            .client
            .patch(
                addressed_route(
                    cluster
                        .routes()
                        .public
                        .patch_note(Uuid::new_v4().to_string())
                )
                .url()
            )
            .json(&json!({ "body": "second draft" }))
            .send()
            .await
            .expect("the cluster answers")
            .status(),
        StatusCode::NOT_FOUND
    );

    cluster.close().await;
}
