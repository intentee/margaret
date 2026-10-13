use margaret::framework::macros::verifies_tokens_from_issuer;
use margaret::framework::trusted_issuer::issuer_keys::IssuerKeys;

#[verifies_tokens_from_issuer(
    ci,
    audience = "https://provider.fixture",
    issuer = "https://ci.fixture",
    keys = IssuerKeys::Discovered,
)]
pub struct CiIssuer;
