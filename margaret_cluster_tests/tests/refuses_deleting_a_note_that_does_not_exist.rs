use reqwest::StatusCode;
use uuid::Uuid;

use margaret_cluster_fixture::margaret::routes::props::DeleteNoteRouteProps;
use margaret_cluster_tests::addressed_route::addressed_route;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::path_parameter::path_parameter;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn refuses_deleting_a_note_that_does_not_exist() {
    let cluster = Cluster::start(cluster_binary(), 1).await;

    assert_eq!(
        cluster
            .client
            .delete(
                addressed_route(cluster.routes().public.delete_note.address(
                    DeleteNoteRouteProps {
                        note: path_parameter(Uuid::new_v4().to_string())
                    }
                ))
                .url()
            )
            .send()
            .await
            .expect("the cluster answers")
            .status(),
        StatusCode::NOT_FOUND
    );

    cluster.close().await;
}
