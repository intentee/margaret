use margaret::framework::macros::verifies_tokens_from_issuer;
use margaret::framework::trusted_issuer::issuer_keys::IssuerKeys;

#[verifies_tokens_from_issuer(
    identity,
    audience = "browser",
    issuer = "https://identity.fixture",
    keys = IssuerKeys::Discovered
)]
pub struct IdentityIssuer;
