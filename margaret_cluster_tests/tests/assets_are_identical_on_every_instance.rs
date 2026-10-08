use reqwest::StatusCode;

use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn assets_are_identical_on_every_instance() {
    let cluster = Cluster::start(cluster_binary(), 3).await;

    for index in 0..3 {
        let response = cluster
            .client
            .get(
                cluster
                    .instance_routes(index)
                    .public
                    .get_asset("cluster_C1A2B3C4.css".to_string())
                    .url(),
            )
            .send()
            .await
            .expect("the asset is fetched");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.bytes().await.expect("the asset is read"),
            include_bytes!("../../margaret_cluster_fixture/assets/cluster_C1A2B3C4.css").as_slice()
        );
    }

    cluster.close().await;
}
