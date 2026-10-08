use margaret::framework::macros::issues_tokens;

#[issues_tokens(
    provider,
    audience = "margaret-cluster",
    issuer = "https://localhost:20443"
)]
pub struct Issuer;
