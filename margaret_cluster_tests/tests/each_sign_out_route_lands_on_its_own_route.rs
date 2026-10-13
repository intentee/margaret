use reqwest::StatusCode;
use reqwest::header::LOCATION;

use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;

async fn landing_of(cluster: &Cluster, sign_out_url: String) -> String {
    let answer = cluster
        .client
        .post(sign_out_url)
        .send()
        .await
        .expect("the sign-out is answered");

    assert_eq!(answer.status(), StatusCode::SEE_OTHER);

    answer
        .headers()
        .get(LOCATION)
        .expect("the sign-out redirects")
        .to_str()
        .expect("the landing is a visible header value")
        .to_string()
}

#[tokio::test]
async fn each_sign_out_route_lands_on_its_own_route() {
    let cluster = Cluster::start(cluster_binary(), 1).await;
    let instance = cluster.instance_routes(0);
    let configured = cluster.routes();

    assert_eq!(
        landing_of(&cluster, instance.public.post_sign_out.url()).await,
        configured.public.get_health.url()
    );
    assert_eq!(
        landing_of(&cluster, instance.identity.post_staff_sign_out.url()).await,
        configured.public.get_notes.url()
    );

    cluster.close().await;
}
