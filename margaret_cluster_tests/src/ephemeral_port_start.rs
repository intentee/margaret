use std::fs;

/// # Panics
///
/// Panics when the kernel does not report the range it assigns ephemeral ports from.
#[must_use]
pub fn ephemeral_port_start() -> u16 {
    fs::read_to_string("/proc/sys/net/ipv4/ip_local_port_range")
        .expect("the ephemeral port range is readable")
        .split_whitespace()
        .next()
        .expect("the ephemeral port range names its first port")
        .parse()
        .expect("the first ephemeral port is a port number")
}
