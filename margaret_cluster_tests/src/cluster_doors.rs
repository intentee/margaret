use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::sync::Arc;

use rustls::ServerConfig;
use url::Url;

use crate::cluster_server::ClusterServer;
use crate::front_door_url::front_door_url;
use crate::relay_destination::RelayDestination;
use crate::tls_relay::TlsRelay;

pub struct ClusterDoors {
    identity: TlsRelay,
    public: TlsRelay,
}

impl ClusterDoors {
    pub async fn open(
        server_config: &Arc<ServerConfig>,
        identity_port: u16,
        destination: impl Fn(ClusterServer) -> RelayDestination,
    ) -> Self {
        Self {
            identity: TlsRelay::open(
                SocketAddr::from((Ipv4Addr::LOCALHOST, identity_port)),
                Arc::clone(server_config),
                destination(ClusterServer::Identity),
            )
            .await,
            public: TlsRelay::open(
                SocketAddr::from((Ipv4Addr::LOCALHOST, 0)),
                Arc::clone(server_config),
                destination(ClusterServer::Public),
            )
            .await,
        }
    }

    #[must_use]
    pub fn url(&self, server: ClusterServer) -> Url {
        front_door_url(match server {
            ClusterServer::Identity => self.identity.port(),
            ClusterServer::Public => self.public.port(),
        })
    }

    pub async fn close(self) {
        self.identity.close().await;
        self.public.close().await;
    }
}
