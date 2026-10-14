use reqwest::StatusCode;
use uuid::Uuid;

use margaret_cluster_fixture::margaret::routes::props::GetNoteViewRouteProps;
use margaret_cluster_tests::addressed_route::addressed_route;
use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn refuses_a_view_of_a_note_that_does_not_exist() {
    let cluster = Cluster::start(cluster_binary(), 1).await;

    assert_eq!(
        cluster
            .client
            .get(
                addressed_route(cluster.routes().public.get_note_view.address(
                    GetNoteViewRouteProps {
                        note: Uuid::new_v4().to_string()
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
