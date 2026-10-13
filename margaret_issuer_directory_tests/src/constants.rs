use margaret_issuer_directory::discovered_issuer::DiscoveredIssuer;
use margaret_issuer_directory::jwks_endpoint_issuer::JwksEndpointIssuer;

pub const LOCALHOST_DISCOVERED_ISSUER: DiscoveredIssuer = DiscoveredIssuer {
    discovery_url: "https://localhost/.well-known/openid-configuration",
    issuer: "https://localhost",
};

pub const LOCALHOST_JWKS_ENDPOINT_ISSUER: JwksEndpointIssuer = JwksEndpointIssuer {
    issuer: "https://localhost",
    jwks_uri: "https://localhost/jwks",
};
