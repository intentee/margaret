use std::io::ErrorKind;
use std::iter;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::sync::OnceLock;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;

use margaret_cluster_fixture::margaret::routes::Routes;

use crate::cluster_server::ClusterServer;
use crate::ephemeral_port_start::ephemeral_port_start;
use crate::first_unprivileged_port::FIRST_UNPRIVILEGED_PORT;

static NEXT_CANDIDATE: OnceLock<AtomicU16> = OnceLock::new();

fn reserved_address() -> SocketAddr {
    let cursor = NEXT_CANDIDATE.get_or_init(|| AtomicU16::new(ephemeral_port_start()));

    iter::from_fn(|| Some(cursor.fetch_sub(1, Ordering::SeqCst) - 1))
        .take_while(|candidate| *candidate >= FIRST_UNPRIVILEGED_PORT)
        .map(|candidate| TcpListener::bind((Ipv4Addr::LOCALHOST, candidate)))
        .find(|bound| !matches!(bound, Err(error) if error.kind() == ErrorKind::AddrInUse))
        .expect("an unprivileged port below the ephemeral range is free")
        .and_then(|listener| listener.local_addr())
        .expect("the reserved port is bound")
}

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
    pub fn routes(&self) -> Routes {
        Routes::from_origins(
            &format!("http://{}", self.identity),
            &format!("http://{}", self.public),
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::net::Ipv4Addr;
    use std::net::TcpListener;

    use super::InstancePorts;
    use crate::ephemeral_port_start::ephemeral_port_start;

    #[test]
    fn reserves_ports_the_system_never_assigns_ephemerally() {
        let ports = InstancePorts::reserve();

        assert!(ports.identity.port() < ephemeral_port_start());
        assert!(ports.public.port() < ephemeral_port_start());
    }

    #[test]
    fn skips_a_candidate_port_another_listener_holds() {
        let first = InstancePorts::reserve();
        let held = TcpListener::bind((Ipv4Addr::LOCALHOST, first.public.port() - 1))
            .expect("the next candidate port is free");
        let second = InstancePorts::reserve();

        assert_eq!(
            held.local_addr().expect("the held port is known").port() - 1,
            second.identity.port()
        );
    }

    #[test]
    fn reserves_distinct_ports_for_every_instance() {
        let first = InstancePorts::reserve();
        let second = InstancePorts::reserve();

        assert_eq!(
            [
                first.identity.port(),
                first.public.port(),
                second.identity.port(),
                second.public.port(),
            ]
            .iter()
            .collect::<BTreeSet<_>>()
            .len(),
            4
        );
    }
}
