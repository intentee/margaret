use futures_util::future::join_all;
use reqwest::StatusCode;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::instance_key_set::instance_key_set;
use margaret_cluster_tests::started_alice_session::started_alice_session;

use crate::cluster_binary::cluster_binary;
use crate::overdue_signing_keys::overdue_signing_keys;
use crate::profile_status::profile_status;
use crate::signing_key_generations::signing_key_generations;

#[tokio::test]
async fn instances_started_on_overdue_keys_roll_once_and_admit_earlier_tokens() {
    let mut cluster = Cluster::start(cluster_binary(), 1).await;
    let earlier = started_alice_session(&cluster, &cluster.instance_routes(0)).await;

    assert!(cluster.stop(0).await.success());

    overdue_signing_keys(&cluster).await;

    let second = cluster.add_member().await;
    let third = cluster.add_member().await;

    cluster.restart(&[0, second, third]).await;

    let documents = join_all((0..3).map(|index| instance_key_set(&cluster, index))).await;

    assert_eq!(signing_key_generations(&cluster).await, vec![2]);
    assert_eq!(documents[1], documents[0]);
    assert_eq!(documents[2], documents[0]);

    for index in 0..3 {
        assert_eq!(
            profile_status(&cluster, index, &earlier.access).await,
            StatusCode::OK
        );
    }

    cluster.close().await;
}
