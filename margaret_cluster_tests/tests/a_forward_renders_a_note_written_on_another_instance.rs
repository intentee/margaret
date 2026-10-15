use margaret_cluster_fixture::margaret::routes::props::GetForwardedNoteRouteProps;
use margaret_cluster_tests::addressed_route::addressed_route;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::note_written::note_written;
use margaret_cluster_tests::path_parameter::path_parameter;
use margaret_cluster_tests::stored_note::StoredNote;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn a_forward_renders_a_note_written_on_another_instance() {
    let cluster = Cluster::start(cluster_binary(), 2).await;
    let id = note_written(&cluster, &cluster.instance_routes(0), "forwarded").await;

    assert_eq!(
        cluster
            .client
            .get(
                addressed_route(
                    cluster
                        .instance_routes(1)
                        .public
                        .get_forwarded_note
                        .address(GetForwardedNoteRouteProps {
                            note_id: path_parameter(id.to_string())
                        })
                )
                .url()
            )
            .send()
            .await
            .expect("the forwarded note is read")
            .json::<StoredNote>()
            .await
            .expect("the forward answers with the note"),
        StoredNote {
            body: "forwarded".to_string(),
            id,
        }
    );

    cluster.close().await;
}
