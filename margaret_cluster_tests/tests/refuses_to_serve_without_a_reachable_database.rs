use margaret_cluster_tests::instance_ports::InstancePorts;

use margaret_cluster_tests::unreachable_database_command::unreachable_database_command;

use crate::cluster_binary::cluster_binary;

#[test]
fn refuses_to_serve_without_a_reachable_database() {
    let ports = InstancePorts::reserve();

    assert!(
        !unreachable_database_command(cluster_binary())
            .arg("serve")
            .arg("--identity-addr")
            .arg(ports.identity.to_string())
            .arg("--public-addr")
            .arg(ports.public.to_string())
            .arg("--public-url")
            .arg(format!("https://{}", ports.public))
            .status()
            .expect("the cluster binary runs")
            .success()
    );
}
