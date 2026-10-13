use std::net::IpAddr;

use serde::Deserialize;
use tokio::process::Command;

use crate::docker_output::docker_output;

#[derive(Debug, Deserialize)]
struct AddressConfig {
    #[serde(rename = "Gateway")]
    gateway: IpAddr,
}

#[derive(Debug, Deserialize)]
struct AddressManagement {
    #[serde(rename = "Config")]
    config: Vec<AddressConfig>,
}

#[derive(Debug, Deserialize)]
struct InspectedNetwork {
    #[serde(rename = "IPAM")]
    address_management: AddressManagement,
}

/// # Panics
///
/// Panics when docker does not describe its default bridge network with exactly one gateway.
pub async fn docker_bridge_gateway() -> IpAddr {
    let networks = serde_json::from_slice::<Vec<InspectedNetwork>>(
        &docker_output(
            Command::new("docker").args(["network", "inspect", "bridge"]),
            "docker inspects its bridge network",
        )
        .await,
    )
    .expect("docker describes its bridge network as json");
    let [network]: [InspectedNetwork; 1] = networks
        .try_into()
        .expect("docker describes exactly one bridge network");
    let [config]: [AddressConfig; 1] = network
        .address_management
        .config
        .try_into()
        .expect("the bridge network has exactly one address configuration");

    config.gateway
}
