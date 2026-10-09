use margaret_cluster_fixture::stores::alice::ALICE;
use margaret_cluster_tests::alice_session_cookie::alice_session_cookie;
use margaret_cluster_tests::bearer_subject::bearer_subject;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::minted_tokens::minted_tokens;
use margaret_cluster_tests::session_refresh_token::session_refresh_token;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn a_session_refresh_token_from_one_instance_mints_on_another() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let refresh_token = session_refresh_token(
        &cluster,
        &cluster.instance_routes(1),
        &alice_session_cookie(),
    )
    .await;
    let tokens = minted_tokens(&cluster, &cluster.instance_routes(2), refresh_token).await;

    assert_eq!(
        bearer_subject(
            &cluster,
            cluster.instance_routes(0).public.get_holder.url(),
            &tokens.access_token,
        )
        .await,
        ALICE.to_string()
    );

    cluster.close().await;
}
