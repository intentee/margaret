use margaret_cluster_fixture::stores::alice::ALICE;
use margaret_cluster_tests::alice_session_cookie::alice_session_cookie;
use margaret_cluster_tests::bearer_subject::bearer_subject;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::minted_tokens::minted_tokens;
use margaret_cluster_tests::session_refresh_token::session_refresh_token;

use crate::cluster_binary::cluster_binary;
use crate::overdue_signing_keys::overdue_signing_keys;
use crate::signing_key_generations::signing_key_generations;

#[tokio::test]
async fn a_refresh_token_signed_after_a_roll_mints_on_an_instance_started_before_it() {
    let mut cluster = Cluster::start(cluster_binary(), 2).await;

    assert!(cluster.stop(1).await.success());

    overdue_signing_keys(&cluster).await;
    cluster.restart(&[1]).await;

    assert_eq!(signing_key_generations(&cluster).await, vec![2]);

    let refresh_token = session_refresh_token(
        &cluster,
        &cluster.instance_routes(1),
        &alice_session_cookie(),
    )
    .await;
    let minted = minted_tokens(&cluster, &cluster.instance_routes(0), refresh_token).await;

    assert_eq!(
        bearer_subject(
            &cluster,
            cluster.instance_routes(1).public.get_holder.url(),
            &minted.access_token,
        )
        .await,
        ALICE.to_string()
    );

    cluster.close().await;
}
