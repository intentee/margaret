use margaret_jwks_keygen::token_malformation::TokenMalformation;

use crate::minted_tokens::MintedTokens;

pub enum AccessTokenMinting {
    ExpiredRefreshToken,
    MalformedRefreshToken(TokenMalformation),
    Minted(MintedTokens),
    UnknownRefreshTokenKey,
}
