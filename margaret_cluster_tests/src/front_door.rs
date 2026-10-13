use std::sync::Arc;

use rustls::ServerConfig;

use crate::cluster_doors::ClusterDoors;
use crate::front_door_backends::FrontDoorBackends;
use crate::relay_destination::RelayDestination;

pub struct FrontDoor {
    pub backends: Arc<FrontDoorBackends>,
    pub doors: ClusterDoors,
}

impl FrontDoor {
    pub async fn open(server_config: &Arc<ServerConfig>, identity_port: u16) -> Self {
        let backends = Arc::new(FrontDoorBackends::default());

        Self {
            doors: ClusterDoors::open(server_config, identity_port, |server| {
                RelayDestination::Admitted {
                    backends: Arc::clone(&backends),
                    server,
                }
            })
            .await,
            backends,
        }
    }
}
