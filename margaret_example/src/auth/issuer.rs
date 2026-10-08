use margaret::framework::macros::issues_tokens;

#[issues_tokens(
    provider,
    audience = "margaret-example",
    issuer = "https://issuer.internal"
)]
pub struct Issuer;
