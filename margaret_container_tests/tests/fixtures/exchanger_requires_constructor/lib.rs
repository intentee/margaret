use margaret::framework::subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;

#[exchanges_tokens_from(issuer = ci)]
#[singleton]
struct CiExchanger {
    url: String,
}

impl ExchangesSubjectTokens for CiExchanger {}
