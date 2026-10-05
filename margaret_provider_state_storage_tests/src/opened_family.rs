use margaret_provider_state_storage::refresh_family::RefreshFamily;
use margaret_token_digest::token_digest::TokenDigest;

pub struct OpenedFamily {
    pub code: TokenDigest,
    pub family: RefreshFamily,
    pub token: TokenDigest,
}
