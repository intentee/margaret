use reqwest::StatusCode;
use reqwest::header::COOKIE;

use margaret_cluster_tests::alice_session_cookie::alice_session_cookie;
use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;
use crate::profile_status::profile_status;

#[tokio::test]
async fn a_sign_out_on_one_instance_is_refused_everywhere() {
    let cluster = Cluster::start(cluster_binary(), 3).await;

    assert_eq!(
        profile_status(&cluster, 1, &alice_session_cookie()).await,
        StatusCode::OK
    );
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
            profile_status(&cluster, index, &alice_session_cookie()).await,
            StatusCode::UNAUTHORIZED
        );
    }

    cluster.close().await;
}
