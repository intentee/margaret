use margaret::framework::macros::trusts_oidc_issuer;

#[trusts_oidc_issuer(partner, audience = "fixture", issuer = "https://partner.fixture")]
pub struct Issuer;
