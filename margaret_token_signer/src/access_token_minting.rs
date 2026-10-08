use margaret_jwt_verification::jwt_rejection::JwtRejection;

use crate::minted_tokens::MintedTokens;

pub enum AccessTokenMinting {
    Minted(MintedTokens),
    RejectedRefreshToken(JwtRejection),
}
