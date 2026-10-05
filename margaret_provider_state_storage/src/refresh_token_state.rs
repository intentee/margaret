use uuid::Uuid;

use crate::refresh_token_standing::RefreshTokenStanding;

#[derive(Clone)]
pub(crate) struct RefreshTokenState {
    pub(crate) family: Uuid,
    pub(crate) standing: RefreshTokenStanding,
}
