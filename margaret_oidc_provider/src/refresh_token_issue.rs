use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_token_digest::token_digest::TokenDigest;

pub(crate) enum RefreshTokenIssue {
    Issued(String),
    Withheld,
}

impl RefreshTokenIssue {
    pub(crate) fn issuance(&self) -> RefreshIssuance {
        match self {
            Self::Issued(token) => RefreshIssuance::Opened(TokenDigest::of(token)),
            Self::Withheld => RefreshIssuance::Withheld,
        }
    }
}
