use margaret::framework::macros::verifies_tokens_from_issuer;
use margaret::framework::trusted_issuer::issuer_keys::IssuerKeys;

#[verifies_tokens_from_issuer(upstream, audience = "fixture", issuer = "https://upstream.fixture", keys = IssuerKeys::Discovered)]
pub struct Issuer;
