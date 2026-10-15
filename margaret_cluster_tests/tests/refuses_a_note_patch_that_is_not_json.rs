use reqwest::StatusCode;

use margaret_cluster_fixture::margaret::routes::props::PatchNoteRouteProps;
use margaret_cluster_tests::addressed_route::addressed_route;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::note_written::note_written;
use margaret_cluster_tests::path_parameter::path_parameter;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn refuses_a_note_patch_that_is_not_json() {
    let cluster = Cluster::start(cluster_binary(), 1).await;
    let id = note_written(&cluster, &cluster.routes(), "first draft").await;

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
                            note: path_parameter(id.to_string())
                        })
                )
                .url()
            )
            .body("second draft")
            .send()
            .await
            .expect("the cluster answers")
            .status(),
        StatusCode::UNSUPPORTED_MEDIA_TYPE
    );

    cluster.close().await;
}
