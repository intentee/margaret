use reqwest::StatusCode;
use uuid::Uuid;

use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_cluster_tests::addressed_route::addressed_route;
use margaret_cluster_tests::cluster::Cluster;
use margaret_database_tests::table_privilege::TablePrivilege;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn fails_downloading_an_upload_the_database_refuses() {
    let cluster = Cluster::start(cluster_binary(), 1).await;
    cluster
        .database
        .administration
        .revoke(
            TablePrivilege::Select,
            TableNamespace::Application,
            "uploads",
        )
        .await;

    assert_eq!(
        cluster
            .client
            .get(
                addressed_route(
                    cluster
                        .routes()
                        .public
                        .get_upload(Uuid::new_v4().to_string())
                )
                .url()
            )
            .send()
            .await
            .expect("the cluster answers")
            .status(),
        StatusCode::INTERNAL_SERVER_ERROR
    );

    cluster.close().await;
}
