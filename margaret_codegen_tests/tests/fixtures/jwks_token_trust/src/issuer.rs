use margaret::framework::macros::issues_tokens;

#[issues_tokens(provider, audience = "fixture", issuer = "https://issuer.fixture")]
pub struct Issuer;
