#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiscoveredIssuer {
    pub discovery_url: &'static str,
    pub issuer: &'static str,
}
