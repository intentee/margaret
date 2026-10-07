use margaret::framework::macros::provides_jwks_endpoint;

#[provides_jwks_endpoint(
    partner,
    audience = "fixture",
    issuer = "https://partner.fixture",
    jwks_uri = "https://partner.fixture/.well-known/jwks.json"
)]
pub struct PartnerEndpoint;
