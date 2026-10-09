use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;
use crate::stored_note::StoredNote;

#[tokio::test]
async fn a_console_command_write_is_visible_on_every_instance() {
    let cluster = Cluster::start(cluster_binary(), 2).await;

    assert!(
        cluster
            .command(&["write-note", "from the console"])
            .await
            .status
            .success()
    );

    for index in 0..2 {
        let notes: Vec<StoredNote> = cluster
            .client
            .get(cluster.instance_routes(index).public.get_notes.url())
            .send()
            .await
            .expect("the notes are listed")
            .json()
            .await
            .expect("the notes are answered");

        assert_eq!(
            notes
                .iter()
                .map(|note| note.body.as_str())
                .collect::<Vec<_>>(),
            vec!["from the console"]
        );
    }

    cluster.close().await;
}
