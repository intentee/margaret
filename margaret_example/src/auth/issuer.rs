use margaret::framework::macros::issues_tokens;

#[issues_tokens(audience = "margaret-example", issuer = "https://issuer.internal")]
pub struct Issuer;
