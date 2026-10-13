use bytes::Bytes;
use futures_util::stream;
use reqwest::Body;
use reqwest::StatusCode;
use reqwest::header::CONTENT_LENGTH;

use margaret_cluster_tests::cluster::Cluster;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn refuses_an_upload_beyond_its_limit() {
    let cluster = Cluster::start(cluster_binary(), 1).await;

    assert_eq!(
        cluster
            .client
            .post(cluster.routes().public.post_upload.url())
            .header(CONTENT_LENGTH, 8_388_609)
            .body(Body::wrap_stream(stream::pending::<
                Result<Bytes, std::io::Error>,
            >()))
            .send()
            .await
            .expect("the cluster answers before the upload is sent")
            .status(),
        StatusCode::PAYLOAD_TOO_LARGE
    );

    cluster.close().await;
}
