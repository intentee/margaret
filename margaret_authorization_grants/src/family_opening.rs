use margaret_token_digest::token_digest::TokenDigest;

use crate::refresh_family::RefreshFamily;

pub struct FamilyOpening {
    pub family: RefreshFamily,
    pub first_token: TokenDigest,
}
