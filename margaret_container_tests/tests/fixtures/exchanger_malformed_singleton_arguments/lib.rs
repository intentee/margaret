use margaret::framework::subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;

#[exchanges_subject_tokens(issuer = ci)]
#[singleton(= 5)]
struct CiExchanger;

impl CiExchanger {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

impl ExchangesSubjectTokens for CiExchanger {}
