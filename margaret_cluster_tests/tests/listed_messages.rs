use margaret_cluster_tests::cluster::Cluster;

use crate::stored_message::StoredMessage;

pub async fn listed_messages(cluster: &Cluster, index: usize) -> Vec<String> {
    cluster
        .client
        .get(cluster.instance_routes(index).public.get_messages.url())
        .send()
        .await
        .expect("the messages are listed")
        .json::<Vec<StoredMessage>>()
        .await
        .expect("the messages are answered")
        .into_iter()
        .map(|message| message.body)
        .collect()
}
