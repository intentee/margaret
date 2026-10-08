use reqwest::StatusCode;
use reqwest::header::COOKIE;

use margaret_cluster_tests::alice_session_cookie::alice_session_cookie;
use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;

async fn profile_status(cluster: &Cluster, index: usize) -> StatusCode {
    cluster
        .client
        .get(cluster.instance_routes(index).public.get_profile.url())
        .header(COOKIE, alice_session_cookie())
        .send()
        .await
        .expect("the profile request is answered")
        .status()
}

#[tokio::test]
async fn a_sign_out_on_one_instance_is_refused_everywhere() {
    let cluster = Cluster::start(cluster_binary(), 3).await;

    assert_eq!(profile_status(&cluster, 1).await, StatusCode::OK);
    assert_eq!(
        cluster
            .client
            .post(cluster.instance_routes(0).public.post_sign_out.url())
            .header(COOKIE, alice_session_cookie())
            .send()
            .await
            .expect("the sign-out is answered")
            .status(),
        StatusCode::OK
    );

    for index in 1..3 {
        assert_eq!(
            profile_status(&cluster, index).await,
            StatusCode::UNAUTHORIZED
        );
    }

    cluster.close().await;
}
