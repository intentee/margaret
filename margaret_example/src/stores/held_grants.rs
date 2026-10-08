use moka::future::Cache;
use uuid::Uuid;

use margaret::framework::authorization_grants::authorization_code_lifetime::AUTHORIZATION_CODE_LIFETIME;
use margaret::framework::authorization_grants::pending_authorization::PendingAuthorization;
use margaret::framework::authorization_grants::pending_authorization_lifetime::PENDING_AUTHORIZATION_LIFETIME;
use margaret::framework::authorization_grants::refresh_family_lifetime::REFRESH_FAMILY_LIFETIME;
use margaret::framework::authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret::framework::token_digest::token_digest::TokenDigest;

use crate::stores::held_code::HeldCode;
use crate::stores::held_family::HeldFamily;
use crate::stores::held_refresh_token::HeldRefreshToken;
use crate::stores::refresh_token_standing::RefreshTokenStanding;

pub struct HeldGrants {
    pub codes: Cache<TokenDigest, HeldCode>,
    pub families: Cache<Uuid, HeldFamily>,
    pub pending: Cache<Uuid, PendingAuthorization>,
    pub refresh_tokens: Cache<TokenDigest, HeldRefreshToken>,
}

impl HeldGrants {
    #[must_use]
    pub fn create() -> Self {
        Self {
            codes: Cache::builder()
                .time_to_live(AUTHORIZATION_CODE_LIFETIME)
                .build(),
            families: Cache::builder()
                .time_to_live(REFRESH_FAMILY_LIFETIME)
                .build(),
            pending: Cache::builder()
                .time_to_live(PENDING_AUTHORIZATION_LIFETIME)
                .build(),
            refresh_tokens: Cache::builder()
                .time_to_live(REFRESH_FAMILY_LIFETIME)
                .build(),
        }
    }

    pub async fn lookup(&self, token: &TokenDigest) -> RefreshTokenLookup {
        match self.refresh_tokens.get(token).await {
            Some(HeldRefreshToken { family, standing }) => match self.families.get(&family).await {
                Some(HeldFamily::Open(record)) => match standing {
                    RefreshTokenStanding::Current => RefreshTokenLookup::Current { family, record },
                    RefreshTokenStanding::Superseded => RefreshTokenLookup::Superseded { family },
                },
                Some(HeldFamily::Revoked) | None => RefreshTokenLookup::Unknown,
            },
            None => RefreshTokenLookup::Unknown,
        }
    }
}
