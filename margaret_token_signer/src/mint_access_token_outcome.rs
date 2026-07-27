use crate::minted_tokens::MintedTokens;
use crate::refresh_token_rejection::RefreshTokenRejection;

pub enum MintAccessTokenOutcome {
    Minted(MintedTokens),
    Rejected(RefreshTokenRejection),
}
