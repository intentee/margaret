use margaret::framework::token_issuance::declares_token_issuance::DeclaresTokenIssuance;

#[singleton]
#[issues_tokens]
struct Issuer;

impl DeclaresTokenIssuance for Issuer {}
