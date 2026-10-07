use margaret::framework::macros::trusts_oidc_issuer;

#[trusts_oidc_issuer(
    ci,
    audience = "https://provider.fixture",
    issuer = "https://ci.fixture"
)]
pub struct CiIssuer;
