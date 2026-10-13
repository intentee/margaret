use margaret_issuer_directory::jwks_endpoint_issuer::JwksEndpointIssuer;

pub const LOCALHOST_JWKS_ENDPOINT_ISSUER: JwksEndpointIssuer = JwksEndpointIssuer {
    issuer: "https://localhost",
    jwks_uri: "https://localhost/jwks",
};
