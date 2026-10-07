use margaret::framework::macros::issues_tokens;

#[issues_tokens(audience = "session", issuer = "https://provider.fixture")]
pub struct ProviderIssuer;
