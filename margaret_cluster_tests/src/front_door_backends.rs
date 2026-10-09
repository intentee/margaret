use std::net::SocketAddr;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use crate::cluster_server::ClusterServer;
use crate::instance_ports::InstancePorts;

#[derive(Default)]
pub struct FrontDoorBackends {
    admitted: Mutex<Vec<InstancePorts>>,
    cursor: AtomicUsize,
}

impl FrontDoorBackends {
    /// # Panics
    ///
    /// Panics when a relay panicked while choosing a backend.
    pub fn admit(&self, ports: InstancePorts) {
        self.admitted
            .lock()
            .expect("the admitted backends are readable")
            .push(ports);
    }

    /// # Panics
    ///
    /// Panics when a relay panicked while choosing a backend.
    pub fn drain(&self, ports: InstancePorts) {
        self.admitted
            .lock()
            .expect("the admitted backends are readable")
            .retain(|admitted| *admitted != ports);
    }

    pub(crate) fn next(&self, server: ClusterServer) -> Option<SocketAddr> {
        let admitted = self
            .admitted
            .lock()
            .expect("the admitted backends are readable");

        admitted
            .get(self.cursor.fetch_add(1, Ordering::Relaxed) % admitted.len().max(1))
            .map(|ports| ports.address(server))
    }
}
