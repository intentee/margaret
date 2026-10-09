use std::net::SocketAddr;
use std::net::TcpListener;
use std::sync::Arc;

use margaret_cluster_fixture::margaret::routes::Routes;

use crate::cluster_server::ClusterServer;

fn reserved_listener() -> TcpListener {
    TcpListener::bind("127.0.0.1:0").expect("a free port is reserved")
}

fn reserved_address(listener: &TcpListener) -> SocketAddr {
    listener.local_addr().expect("the reserved port is known")
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct InstancePorts {
    pub identity: SocketAddr,
    pub public: SocketAddr,
}

impl InstancePorts {
    #[must_use]
    pub fn reserve() -> Self {
        let identity = reserved_listener();
        let public = reserved_listener();

        Self {
            identity: reserved_address(&identity),
            public: reserved_address(&public),
        }
    }

    #[must_use]
    pub fn routes(&self) -> Routes {
        Routes::from_origins(
            Arc::from(format!("http://{}", self.identity)),
            Arc::from(format!("http://{}", self.public)),
        )
    }

    #[must_use]
    pub fn address(&self, server: ClusterServer) -> SocketAddr {
        match server {
            ClusterServer::Identity => self.identity,
            ClusterServer::Public => self.public,
        }
    }
}
