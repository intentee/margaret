use reqwest::StatusCode;
use serde_json::json;
use uuid::Uuid;

use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_cluster_fixture::margaret::routes::props::PatchNoteRouteProps;
use margaret_cluster_tests::addressed_route::addressed_route;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::path_parameter::path_parameter;
use margaret_database_tests::table_privilege::TablePrivilege;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn fails_patching_a_note_the_database_refuses() {
    let cluster = Cluster::start(cluster_binary(), 1).await;
    cluster
        .database
        .administration
        .revoke(TablePrivilege::Select, TableNamespace::Application, "notes")
        .await;

    assert_eq!(
        cluster
            .client
            .patch(
                addressed_route(
                    cluster
                        .routes()
                        .public
                        .patch_note
                        .address(PatchNoteRouteProps {
                            note: path_parameter(Uuid::new_v4().to_string())
                        })
                )
                .url()
            )
            .json(&json!({ "body": "second draft" }))
            .send()
            .await
            .expect("the cluster answers")
            .status(),
        StatusCode::INTERNAL_SERVER_ERROR
    );

    cluster.close().await;
}
