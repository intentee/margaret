use margaret_issuer_directory::discovered_issuer::DiscoveredIssuer;

pub const LOCALHOST_DISCOVERED_ISSUER: DiscoveredIssuer = DiscoveredIssuer {
    discovery_url: "https://localhost/.well-known/openid-configuration",
    issuer: "https://localhost",
};
