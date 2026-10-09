use reqwest::StatusCode;
use reqwest::header::COOKIE;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::response_cookie::response_cookie;
use margaret_cluster_tests::session_access_cookie::SESSION_ACCESS_COOKIE;
use margaret_cluster_tests::started_alice_session::started_alice_session;

use crate::cluster_binary::cluster_binary;
use crate::overdue_signing_keys::overdue_signing_keys;
use crate::profile_status::profile_status;
use crate::signing_key_generations::signing_key_generations;

#[tokio::test]
async fn a_session_signed_after_a_roll_is_admitted_on_an_instance_started_before_it() {
    let mut cluster = Cluster::start(cluster_binary(), 2).await;

    assert!(cluster.stop(1).await.success());

    overdue_signing_keys(&cluster).await;
    cluster.restart(&[1]).await;

    assert_eq!(signing_key_generations(&cluster).await, vec![2]);

    let alice = started_alice_session(&cluster, &cluster.instance_routes(1)).await;

    assert_eq!(
        profile_status(&cluster, 0, &alice.access).await,
        StatusCode::OK
    );

    let refreshed = cluster
        .client
        .get(cluster.instance_routes(0).public.get_profile.url())
        .header(COOKIE, &alice.secret)
        .send()
        .await
        .expect("the profile request is answered");

    assert_eq!(refreshed.status(), StatusCode::OK);
    assert_eq!(
        profile_status(
            &cluster,
            1,
            &response_cookie(&refreshed, SESSION_ACCESS_COOKIE)
        )
        .await,
        StatusCode::OK
    );

    cluster.close().await;
}
