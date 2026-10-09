use margaret_cluster_tests::cluster::Cluster;

use crate::addressed_route::addressed_route;
use crate::cluster_binary::cluster_binary;
use crate::note_written::note_written;

#[tokio::test]
async fn a_view_shows_a_note_written_on_another_instance() {
    let cluster = Cluster::start(cluster_binary(), 2).await;
    let id = note_written(&cluster, &cluster.instance_routes(0), "viewed").await;

    assert_eq!(
        cluster
            .client
            .get(
                addressed_route(
                    cluster
                        .instance_routes(1)
                        .public
                        .get_note_view(id.to_string())
                )
                .url()
            )
            .send()
            .await
            .expect("the note view is read")
            .text()
            .await
            .expect("the view is text"),
        "<article>viewed</article>"
    );

    cluster.close().await;
}
