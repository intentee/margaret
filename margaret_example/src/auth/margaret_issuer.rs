use margaret::framework::macros::trusts_oidc_issuer;

#[trusts_oidc_issuer(margaret, audience = "attachments", issuer = "https://issuer.internal")]
pub struct MargaretIssuer;
