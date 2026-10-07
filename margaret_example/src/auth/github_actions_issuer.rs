use margaret::framework::macros::provides_jwks_endpoint;

#[provides_jwks_endpoint(
    github_actions,
    audience = "https://issuer.internal",
    issuer = "https://token.actions.githubusercontent.com",
    jwks_uri = "https://token.actions.githubusercontent.com/.well-known/jwks"
)]
pub struct GithubActionsIssuer;
