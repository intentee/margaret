use chrono::Utc;
use reqwest::StatusCode;
use serde_json::json;

use margaret_cluster_fixture::constants::NOTE_SWEEP_INTERVAL;
use margaret_cluster_fixture::margaret::routes::props::GetNoteRouteProps;
use margaret_cluster_tests::addressed_route::addressed_route;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::path_parameter::path_parameter;
use margaret_cluster_tests::stored_note::StoredNote;
use margaret_local_server_tests::poll_until::poll_until;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn any_instance_sweeps_notes_another_wrote_so_a_crashed_writer_leaves_no_garbage() {
    let mut cluster = Cluster::start(cluster_binary(), 2).await;
    let StoredNote { id, .. } = cluster
        .client
        .post(cluster.instance_routes(0).public.post_note.url())
        .json(&json!({
            "body": "expiring",
            "expires_at": Utc::now() + NOTE_SWEEP_INTERVAL,
        }))
        .send()
        .await
        .expect("the note is posted")
        .json()
        .await
        .expect("the note is stored");

    cluster.crash(0).await;

    let read_on_survivor = async || {
        cluster
            .client
            .get(
                addressed_route(cluster.instance_routes(1).public.get_note.address(
                    GetNoteRouteProps {
                        note: path_parameter(id.to_string()),
                    },
                ))
                .url(),
            )
            .send()
            .await
            .expect("the note is read")
            .status()
    };

    assert_eq!(read_on_survivor().await, StatusCode::OK);

    poll_until(|| async { read_on_survivor().await == StatusCode::NOT_FOUND }).await;

    cluster.close().await;
}
