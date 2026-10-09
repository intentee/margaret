use std::time::Duration;
use std::time::Instant;

use moka::Expiry;

use margaret_authorization_server_client::token_target::TokenTarget;

use crate::cached_acquisition::CachedAcquisition;

pub(crate) struct TokenExpiry;

impl Expiry<TokenTarget, CachedAcquisition> for TokenExpiry {
    fn expire_after_create(
        &self,
        _target: &TokenTarget,
        cached: &CachedAcquisition,
        _created_at: Instant,
    ) -> Option<Duration> {
        Some(cached.reusable_for)
    }
}
