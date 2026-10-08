use margaret::framework::macros::verifies_tokens_from_issuer;
use margaret::framework::trusted_issuer::issuer_keys::IssuerKeys;

#[verifies_tokens_from_issuer(
    github_actions,
    audience = "https://issuer.internal",
    issuer = "https://token.actions.githubusercontent.com",
    keys = IssuerKeys::Published(
        jwks_uri = "https://token.actions.githubusercontent.com/.well-known/jwks",
    ),
)]
pub struct GithubActionsIssuer;
