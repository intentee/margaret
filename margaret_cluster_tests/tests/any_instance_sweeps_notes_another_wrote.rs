use chrono::Utc;
use reqwest::StatusCode;
use serde_json::json;

use margaret_cluster_fixture::tickers::note_sweep_interval::NOTE_SWEEP_INTERVAL;
use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;
use crate::poll_until::poll_until;
use crate::stored_note::StoredNote;

#[tokio::test]
async fn any_instance_sweeps_notes_another_wrote() {
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
                cluster
                    .instance_routes(1)
                    .public
                    .get_note(id.to_string())
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
