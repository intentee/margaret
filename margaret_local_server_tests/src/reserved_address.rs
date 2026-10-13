use std::io::ErrorKind;
use std::iter;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::sync::OnceLock;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;

use crate::constants::FIRST_UNPRIVILEGED_PORT;
use crate::ephemeral_port_start::ephemeral_port_start;

static NEXT_CANDIDATE: OnceLock<AtomicU16> = OnceLock::new();

/// # Panics
///
/// Panics when every unprivileged port below the ephemeral range is taken.
#[must_use]
pub fn reserved_address() -> SocketAddr {
    let cursor = NEXT_CANDIDATE.get_or_init(|| AtomicU16::new(ephemeral_port_start()));

    iter::from_fn(|| Some(cursor.fetch_sub(1, Ordering::SeqCst) - 1))
        .take_while(|candidate| *candidate >= FIRST_UNPRIVILEGED_PORT)
        .map(|candidate| TcpListener::bind((Ipv4Addr::LOCALHOST, candidate)))
        .find(|bound| !matches!(bound, Err(error) if error.kind() == ErrorKind::AddrInUse))
        .expect("an unprivileged port below the ephemeral range is free")
        .and_then(|listener| listener.local_addr())
        .expect("the reserved port is bound")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::net::Ipv4Addr;
    use std::net::SocketAddr;
    use std::net::TcpListener;

    use super::reserved_address;
    use crate::ephemeral_port_start::ephemeral_port_start;

    #[test]
    fn reserves_a_port_the_system_never_assigns_ephemerally() {
        assert!(reserved_address().port() < ephemeral_port_start());
    }

    #[test]
    fn skips_a_candidate_port_another_listener_holds() {
        let first = reserved_address();
        let held = TcpListener::bind((Ipv4Addr::LOCALHOST, first.port() - 1))
            .expect("the next candidate port is free");

        assert_eq!(
            held.local_addr().expect("the held port is known").port() - 1,
            reserved_address().port()
        );
    }

    #[test]
    fn reserves_distinct_ports_so_servers_never_collide_on_bind() {
        assert_eq!(
            [reserved_address(), reserved_address(), reserved_address()]
                .iter()
                .map(SocketAddr::port)
                .collect::<BTreeSet<u16>>()
                .len(),
            3
        );
    }
}
