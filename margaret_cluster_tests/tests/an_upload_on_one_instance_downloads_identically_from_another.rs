use reqwest::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;

#[derive(Deserialize)]
struct StoredUpload {
    id: Uuid,
}

#[tokio::test]
async fn an_upload_on_one_instance_downloads_identically_from_another() {
    let cluster = Cluster::start(cluster_binary(), 2).await;
    let content: Vec<u8> = (0..=u8::MAX).cycle().take(65_536).collect();
    let response = cluster
        .client
        .post(cluster.instance_routes(0).public.post_upload.url())
        .body(content.clone())
        .send()
        .await
        .expect("the upload is sent");

    assert_eq!(response.status(), StatusCode::CREATED);

    let StoredUpload { id } = response.json().await.expect("the upload is stored");

    assert_eq!(
        cluster
            .client
            .get(
                cluster
                    .instance_routes(1)
                    .public
                    .get_upload(id.to_string())
                    .url()
            )
            .send()
            .await
            .expect("the upload is downloaded")
            .bytes()
            .await
            .expect("the upload is read"),
        content
    );

    cluster.close().await;
}
