use reqwest::StatusCode;
use reqwest::header::COOKIE;

use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::started_alice_session::started_alice_session;

use crate::cluster_binary::cluster_binary;
use crate::profile_status::profile_status;

#[tokio::test]
async fn a_sign_out_on_one_instance_is_refused_everywhere() {
    let cluster = Cluster::start(cluster_binary(), 3).await;
    let alice = started_alice_session(&cluster, &cluster.instance_routes(0)).await;

    assert_eq!(
        profile_status(&cluster, 1, &alice.cookies).await,
        StatusCode::OK
    );
    assert_eq!(
        cluster
            .client
            .post(cluster.instance_routes(0).public.post_sign_out.url())
            .header(COOKIE, &alice.cookies)
            .send()
            .await
            .expect("the sign-out is answered")
            .status(),
        StatusCode::SEE_OTHER
    );

    for index in 1..3 {
        assert_eq!(
            profile_status(&cluster, index, &alice.secret).await,
            StatusCode::UNAUTHORIZED
        );
    }

    cluster.close().await;
}
