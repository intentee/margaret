use margaret::framework::macros::exchanges_tokens_from;

#[exchanges_tokens_from(issuer = ci)]
enum Bad {}
