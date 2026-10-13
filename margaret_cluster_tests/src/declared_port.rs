use url::Url;

/// # Panics
///
/// Panics when the declared URL does not parse or names no port.
#[must_use]
pub fn declared_port(declared_url: &str) -> u16 {
    Url::parse(declared_url)
        .expect("the declared URL parses")
        .port_or_known_default()
        .expect("the declared URL names a port")
}
