use margaret::framework::subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;

#[exchanges_tokens_from(issuer = ci)]
#[singleton(unexpected = Thing)]
struct CiExchanger;

impl CiExchanger {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

impl ExchangesSubjectTokens for CiExchanger {}
