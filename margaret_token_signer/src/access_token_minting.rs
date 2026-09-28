use margaret_jws_verification::jws_rejection::JwsRejection;

use crate::minted_tokens::MintedTokens;

pub enum AccessTokenMinting {
    ExpiredRefreshToken,
    MalformedRefreshTokenClaims(serde_json::Error),
    Minted(MintedTokens),
    RefreshTokenSignedWithNextKey,
    RejectedRefreshToken(JwsRejection),
}
