use margaret::framework::macros::trusts_oidc_issuer;

#[trusts_oidc_issuer(
    auth,
    audience = "margaret-example",
    issuer = "https://issuer.internal"
)]
pub struct SessionIssuer;
