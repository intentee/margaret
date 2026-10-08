use margaret_cluster_fixture::stores::alice::ALICE;
use margaret_cluster_tests::alice_session_cookie::alice_session_cookie;
use margaret_cluster_tests::bearer_subject::bearer_subject;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::minted_tokens::minted_tokens;
use margaret_cluster_tests::session_refresh_token::session_refresh_token;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn a_crashed_instance_restarted_admits_earlier_tokens() {
    let mut cluster = Cluster::start(cluster_binary(), 2).await;
    let routes = cluster.instance_routes(0);
    let refresh_token = session_refresh_token(&cluster, &routes, &alice_session_cookie()).await;
    let tokens = minted_tokens(&cluster, &routes, refresh_token).await;

    cluster.crash(0).await;
    cluster.restart(&[0]).await;

    assert_eq!(
        bearer_subject(
            &cluster,
            routes.public.get_holder.url(),
            &tokens.access_token
        )
        .await,
        ALICE.to_string()
    );

    cluster.close().await;
}
