use margaret::framework::macros::issues_tokens;

#[issues_tokens(provider, audience = "session", issuer = "https://provider.fixture")]
pub struct ProviderIssuer;
