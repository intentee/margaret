use margaret::framework::macros::issues_tokens;

#[issues_tokens(provider, issuer = "https://issuer.fixture")]
struct Issuer;
