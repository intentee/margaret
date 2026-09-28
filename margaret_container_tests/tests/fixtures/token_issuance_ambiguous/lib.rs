use margaret::framework::token_issuance::declares_token_issuance::DeclaresTokenIssuance;

#[singleton]
#[issues_tokens]
struct FirstIssuer;

impl DeclaresTokenIssuance for FirstIssuer {}

#[singleton]
#[issues_tokens]
struct SecondIssuer;

impl DeclaresTokenIssuance for SecondIssuer {}
