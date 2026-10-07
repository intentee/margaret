use margaret::framework::macros::issues_tokens;

#[issues_tokens(audience = "fixture", issuer = "https://issuer.fixture")]
pub struct Issuer;
