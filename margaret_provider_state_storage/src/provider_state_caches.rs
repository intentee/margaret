use moka::future::Cache;
use uuid::Uuid;

use margaret_token_digest::token_digest::TokenDigest;

use crate::authorization_code_lifetime::AUTHORIZATION_CODE_LIFETIME;
use crate::code_state::CodeState;
use crate::live_refresh_token::LiveRefreshToken;
use crate::pending_authorization::PendingAuthorization;
use crate::pending_authorization_lifetime::PENDING_AUTHORIZATION_LIFETIME;
use crate::refresh_family::RefreshFamily;
use crate::refresh_family_lifetime::REFRESH_FAMILY_LIFETIME;
use crate::refresh_token_state::RefreshTokenState;
use crate::spent_assertion::SpentAssertion;
use crate::spent_assertion_expiry::SpentAssertionExpiry;
use crate::spent_assertion_key::SpentAssertionKey;

pub(crate) struct ProviderStateCaches {
    pub(crate) assertions: Cache<SpentAssertionKey, SpentAssertion>,
    pub(crate) codes: Cache<TokenDigest, CodeState>,
    pub(crate) families: Cache<Uuid, RefreshFamily>,
    pub(crate) pending: Cache<Uuid, PendingAuthorization>,
    pub(crate) tokens: Cache<TokenDigest, RefreshTokenState>,
}

impl ProviderStateCaches {
    pub(crate) fn create() -> Self {
        Self {
            assertions: Cache::builder().expire_after(SpentAssertionExpiry).build(),
            codes: Cache::builder()
                .time_to_live(AUTHORIZATION_CODE_LIFETIME)
                .build(),
            families: Cache::builder()
                .time_to_live(REFRESH_FAMILY_LIFETIME)
                .build(),
            pending: Cache::builder()
                .time_to_live(PENDING_AUTHORIZATION_LIFETIME)
                .build(),
            tokens: Cache::builder()
                .time_to_live(REFRESH_FAMILY_LIFETIME)
                .build(),
        }
    }

    pub(crate) async fn live_refresh_token(
        &self,
        presented: &TokenDigest,
    ) -> Option<LiveRefreshToken> {
        let state = self.tokens.get(presented).await?;

        self.families
            .get(&state.family)
            .await
            .map(|family| LiveRefreshToken { family, state })
    }
}
