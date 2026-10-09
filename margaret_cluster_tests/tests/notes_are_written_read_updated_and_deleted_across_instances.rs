use reqwest::StatusCode;
use serde_json::json;

use margaret_cluster_tests::cluster::Cluster;

use crate::addressed_route::addressed_route;
use crate::cluster_binary::cluster_binary;
use crate::note_written::note_written;
use crate::stored_note::StoredNote;

#[tokio::test]
async fn notes_are_written_read_updated_and_deleted_across_instances() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let routes = |index| cluster.instance_routes(index);
    let id = note_written(&cluster, &routes(0), "first draft").await;
    let read = |index: usize| {
        cluster
            .client
            .get(addressed_route(routes(index).public.get_note(id.to_string())).url())
            .send()
    };

    assert_eq!(
        read(1)
            .await
            .expect("the note is read")
            .json::<StoredNote>()
            .await
            .expect("the note is answered"),
        StoredNote {
            body: "first draft".to_string(),
            id,
        }
    );
    assert_eq!(
        cluster
            .client
            .patch(addressed_route(routes(2).public.patch_note(id.to_string())).url())
            .json(&json!({ "body": "second draft" }))
            .send()
            .await
            .expect("the note is updated")
            .status(),
        StatusCode::OK
    );
    assert_eq!(
        read(0)
            .await
            .expect("the note is read")
            .json::<StoredNote>()
            .await
            .expect("the note is answered")
            .body,
        "second draft"
    );
    assert_eq!(
        cluster
            .client
            .delete(addressed_route(routes(1).public.delete_note(id.to_string())).url())
            .send()
            .await
            .expect("the note is deleted")
            .status(),
        StatusCode::OK
    );
    assert_eq!(
        read(2).await.expect("the note is read").status(),
        StatusCode::NOT_FOUND
    );

    cluster.close().await;
}
