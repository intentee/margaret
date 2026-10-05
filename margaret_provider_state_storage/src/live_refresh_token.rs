use crate::refresh_family::RefreshFamily;
use crate::refresh_token_state::RefreshTokenState;

pub(crate) struct LiveRefreshToken {
    pub(crate) family: RefreshFamily,
    pub(crate) state: RefreshTokenState,
}
