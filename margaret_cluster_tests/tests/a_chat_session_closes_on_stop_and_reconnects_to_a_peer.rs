use futures_util::StreamExt as _;

use margaret_cluster_tests::chat_socket::chat_socket;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::posted_chat_message::posted_chat_message;

use crate::cluster_binary::cluster_binary;
use crate::listed_messages::listed_messages;

#[tokio::test]
async fn a_chat_session_closes_on_stop_and_reconnects_to_a_peer() {
    let mut cluster = Cluster::start(cluster_binary(), 2).await;
    let mut socket = chat_socket(&cluster, &cluster.instance_url(0, ClusterServer::Public)).await;

    assert!(posted_chat_message(&mut socket, "before").await.is_text());
    assert!(cluster.stop(0).await.success());
    assert!(
        socket
            .next()
            .await
            .expect("the stopped instance answers the session")
            .expect("the stopped instance closes the session cleanly")
            .is_close()
    );

    let mut reconnected = chat_socket(&cluster, &cluster.url(ClusterServer::Public)).await;

    assert!(
        posted_chat_message(&mut reconnected, "after")
            .await
            .is_text()
    );
    assert_eq!(listed_messages(&cluster, 1).await, vec!["before", "after"]);

    cluster.close().await;
}
