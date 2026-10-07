use margaret::framework::macros::trusts_oidc_issuer;

#[trusts_oidc_issuer(upstream, audience = "fixture", issuer = "https://upstream.fixture")]
pub struct Issuer;
