use margaret::framework::macros::exchanges_tokens_from;
use margaret::framework::macros::singleton;

#[singleton]
#[exchanges_tokens_from(issuer = ci)]
struct CiExchanger;
