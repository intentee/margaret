use reqwest::StatusCode;
use serde_json::json;
use uuid::Uuid;

use margaret_cluster_fixture::margaret::routes::Routes;
use margaret_cluster_tests::cluster::Cluster;

use crate::stored_note::StoredNote;

pub async fn note_written(cluster: &Cluster, routes: &Routes, body: &str) -> Uuid {
    let response = cluster
        .client
        .post(routes.public.post_note.url())
        .json(&json!({ "body": body }))
        .send()
        .await
        .expect("the note is posted");

    assert_eq!(response.status(), StatusCode::CREATED);

    response
        .json::<StoredNote>()
        .await
        .expect("the stored note is answered")
        .id
}
