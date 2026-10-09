use margaret::framework::macros::issues_tokens;

#[issues_tokens(provider, issuer = "https://provider.fixture")]
pub struct ProviderIssuer;
