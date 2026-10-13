use margaret::framework::macros::issues_tokens;

#[issues_tokens(provider, issuer = "https://localhost:20443")]
pub struct Issuer;
