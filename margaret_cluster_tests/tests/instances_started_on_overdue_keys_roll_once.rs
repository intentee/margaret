use futures_util::future::join_all;

use margaret_cluster_fixture::stores::alice::ALICE;
use margaret_cluster_tests::alice_session_cookie::alice_session_cookie;
use margaret_cluster_tests::bearer_subject::bearer_subject;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::instance_key_set::instance_key_set;
use margaret_cluster_tests::minted_tokens::minted_tokens;
use margaret_cluster_tests::session_refresh_token::session_refresh_token;

use crate::cluster_binary::cluster_binary;
use crate::overdue_signing_keys::overdue_signing_keys;
use crate::signing_key_generations::signing_key_generations;

#[tokio::test]
async fn instances_started_on_overdue_keys_roll_once_and_admit_earlier_tokens() {
    let mut cluster = Cluster::start(cluster_binary(), 1).await;
    let routes = cluster.instance_routes(0);
    let refresh_token = session_refresh_token(&cluster, &routes, &alice_session_cookie()).await;
    let earlier = minted_tokens(&cluster, &routes, refresh_token).await;

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
            bearer_subject(
                &cluster,
                cluster.instance_routes(index).public.get_holder.url(),
                &earlier.access_token,
            )
            .await,
            ALICE.to_string()
        );
    }

    cluster.close().await;
}
