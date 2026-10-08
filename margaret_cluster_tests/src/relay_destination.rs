use std::net::SocketAddr;
use std::sync::Arc;

use crate::cluster_server::ClusterServer;
use crate::front_door_backends::FrontDoorBackends;

pub enum RelayDestination {
    Admitted {
        backends: Arc<FrontDoorBackends>,
        server: ClusterServer,
    },
    Fixed(SocketAddr),
}

impl RelayDestination {
    pub(crate) fn next(&self) -> Option<SocketAddr> {
        match self {
            Self::Admitted { backends, server } => backends.next(*server),
            Self::Fixed(address) => Some(*address),
        }
    }
}
