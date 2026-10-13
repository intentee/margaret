use reqwest::StatusCode;
use reqwest::header::COOKIE;

use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::constants::SESSION_SECRET_COOKIE;
use margaret_database_tests::table_privilege::TablePrivilege;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn fails_a_profile_whose_session_cannot_be_resolved() {
    let cluster = Cluster::start(cluster_binary(), 1).await;
    cluster
        .database
        .administration
        .revoke(
            TablePrivilege::Select,
            TableNamespace::Framework,
            "sessions",
        )
        .await;

    assert_eq!(
        cluster
            .client
            .get(cluster.routes().public.get_profile.url())
            .header(COOKIE, format!("{SESSION_SECRET_COOKIE}=forged"))
            .send()
            .await
            .expect("the cluster answers")
            .status(),
        StatusCode::INTERNAL_SERVER_ERROR
    );

    cluster.close().await;
}
