use uuid::Uuid;

use crate::stores::refresh_token_standing::RefreshTokenStanding;

#[derive(Clone, Copy)]
pub struct HeldRefreshToken {
    pub family: Uuid,
    pub standing: RefreshTokenStanding,
}
