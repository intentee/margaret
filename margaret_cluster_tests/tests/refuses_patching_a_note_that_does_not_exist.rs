use reqwest::StatusCode;
use serde_json::json;
use uuid::Uuid;

use margaret_cluster_fixture::margaret::routes::props::PatchNoteRouteProps;
use margaret_cluster_tests::addressed_route::addressed_route;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::path_parameter::path_parameter;

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
                        .patch_note
                        .address(PatchNoteRouteProps {
                            note: path_parameter(Uuid::new_v4().to_string())
                        })
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
