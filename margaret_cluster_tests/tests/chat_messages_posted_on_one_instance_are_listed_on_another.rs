use margaret_cluster_tests::chat_socket::chat_socket;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::posted_chat_message::posted_chat_message;

use crate::cluster_binary::cluster_binary;
use crate::listed_messages::listed_messages;

#[tokio::test]
async fn chat_messages_posted_on_one_instance_are_listed_on_another() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let mut socket = chat_socket(&cluster, &cluster.instance_url(1, ClusterServer::Public)).await;

    assert!(posted_chat_message(&mut socket, "hello").await.is_text());
    assert_eq!(listed_messages(&cluster, 2).await, vec!["hello"]);

    cluster.close().await;
}
