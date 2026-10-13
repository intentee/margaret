use std::net::SocketAddr;

use margaret_cluster_fixture::margaret::routes::Routes;
use margaret_local_server_tests::reserved_address::reserved_address;

use crate::cluster_server::ClusterServer;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct InstancePorts {
    pub identity: SocketAddr,
    pub public: SocketAddr,
}

impl InstancePorts {
    #[must_use]
    pub fn reserve() -> Self {
        Self {
            identity: reserved_address(),
            public: reserved_address(),
        }
    }

    #[must_use]
    pub fn address(&self, server: ClusterServer) -> SocketAddr {
        match server {
            ClusterServer::Identity => self.identity,
            ClusterServer::Public => self.public,
        }
    }

    /// # Panics
    ///
    /// Panics when the routes of the cluster cannot be built.
    #[must_use]
    pub fn routes(&self) -> Routes {
        Routes::from_origins(
            &format!("http://{}", self.identity),
            &format!("http://{}", self.public),
        )
        .expect("the routes of the cluster build")
    }
}

#[cfg(test)]
mod tests {
    use super::InstancePorts;

    #[test]
    fn reserves_distinct_ports_for_the_servers_of_an_instance() {
        let ports = InstancePorts::reserve();

        assert_ne!(ports.identity.port(), ports.public.port());
    }
}
