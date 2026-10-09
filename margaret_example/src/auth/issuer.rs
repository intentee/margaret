use margaret::framework::macros::issues_tokens;

#[issues_tokens(provider, issuer = "https://issuer.internal")]
pub struct Issuer;
