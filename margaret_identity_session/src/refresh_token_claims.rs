use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::ACCESS_TOKEN_LIFETIME_SECS;
use crate::access_token_claims::AccessTokenClaims;

#[derive(Clone, Deserialize, Serialize)]
pub struct RefreshTokenClaims {
    pub exp: i64,
    pub iat: i64,
    pub jti: Uuid,
    pub sub: Uuid,
}

impl RefreshTokenClaims {
    #[must_use]
    pub fn is_expired(&self, now: i64) -> bool {
        self.exp < now
    }

    #[must_use]
    pub fn mint_access_token_clams(&self, now: i64) -> AccessTokenClaims {
        AccessTokenClaims {
            sub: self.sub,
            exp: now + ACCESS_TOKEN_LIFETIME_SECS,
            iat: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::RefreshTokenClaims;
    use crate::ACCESS_TOKEN_LIFETIME_SECS;

    fn claims(exp: i64) -> RefreshTokenClaims {
        RefreshTokenClaims {
            exp,
            iat: 0,
            jti: Uuid::from_u128(2),
            sub: Uuid::from_u128(3),
        }
    }

    #[test]
    fn reports_expiry_relative_to_now() {
        assert!(!claims(101).is_expired(100));
        assert!(claims(99).is_expired(100));
        assert!(!claims(100).is_expired(100));
    }

    #[test]
    fn mints_access_token_claims_for_the_same_subject() {
        let refresh = claims(10_000);
        let now = 1_000;

        let access = refresh.mint_access_token_clams(now);

        assert_eq!(access.sub, refresh.sub);
        assert_eq!(access.iat, now);
        assert_eq!(access.exp, now + ACCESS_TOKEN_LIFETIME_SECS);
    }
}
