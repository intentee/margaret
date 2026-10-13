use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_cluster_tests::cluster::Cluster;
use margaret_cluster_tests::cluster_server::ClusterServer;
use margaret_cluster_tests::instance_ports::InstancePorts;
use margaret_database_tests::table_privilege::TablePrivilege;

use crate::cluster_binary::cluster_binary;

#[tokio::test]
async fn refuses_to_serve_when_its_signing_keys_cannot_be_read() {
    let cluster = Cluster::prepare(cluster_binary()).await;
    let ports = InstancePorts::reserve();

    cluster
        .database
        .administration
        .revoke(
            TablePrivilege::Select,
            TableNamespace::Framework,
            "signing_key_sets",
        )
        .await;

    assert!(
        !cluster
            .command(&[
                "serve",
                "--identity-addr",
                &ports.identity.to_string(),
                "--public-addr",
                &ports.public.to_string(),
                "--public-url",
                cluster.url(ClusterServer::Public).as_str(),
            ])
            .await
            .status
            .success()
    );

    cluster.close().await;
}
