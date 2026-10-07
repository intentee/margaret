use margaret::framework::subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;

#[exchanges_subject_tokens(issuer = ci)]
#[singleton]
struct CiExchanger {
    url: String,
}

impl ExchangesSubjectTokens for CiExchanger {}
