use margaret::framework::macros::verifies_tokens_from_issuer;
use margaret::framework::trusted_issuer::issuer_keys::IssuerKeys;

#[verifies_tokens_from_issuer(
    external,
    audience = "https://localhost:20443",
    issuer = "https://localhost:20444",
    keys = IssuerKeys::Published(jwks_uri = "https://localhost:20444/jwks.json"),
)]
pub struct ExternalIssuer;
