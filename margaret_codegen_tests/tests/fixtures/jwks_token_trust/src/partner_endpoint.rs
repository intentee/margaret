use margaret::framework::macros::verifies_tokens_from_issuer;
use margaret::framework::trusted_issuer::issuer_keys::IssuerKeys;

#[verifies_tokens_from_issuer(
    partner,
    audience = "fixture",
    issuer = "https://partner.fixture",
    keys = IssuerKeys::Published(jwks_uri = "https://partner.fixture/.well-known/jwks.json"),
)]
pub struct PartnerEndpoint;
