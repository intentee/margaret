use margaret::framework::macros::verifies_tokens_from_issuer;
use margaret::framework::trusted_issuer::issuer_keys::IssuerKeys;

#[verifies_tokens_from_issuer(partner, audience = "artifacts", issuer = "https://partner.fixture", keys = IssuerKeys::Discovered)]
pub struct PartnerIssuer;
