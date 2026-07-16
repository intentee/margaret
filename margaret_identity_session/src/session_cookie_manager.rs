use cookie::Cookie;
use cookie::SameSite;
use cookie::time::OffsetDateTime;

use crate::access_token_claims_signed::AccessTokenClaimsSigned;
use crate::cookie_name_access_token::COOKIE_NAME_ACCESS_TOKEN;
use crate::cookie_name_refresh_token::COOKIE_NAME_REFRESH_TOKEN;
use crate::identity_session_error::IdentitySessionError;
use crate::refresh_token_claims_signed::RefreshTokenClaimsSigned;

#[derive(Clone)]
pub struct SessionCookieManager {
    pub cookie_domain: String,
    pub cookie_secure: bool,
}

impl SessionCookieManager {
    pub fn access_token_cookie(
        &self,
        AccessTokenClaimsSigned { exp, signed_claims }: AccessTokenClaimsSigned,
    ) -> Result<Cookie<'static>, IdentitySessionError> {
        Ok(Cookie::build((COOKIE_NAME_ACCESS_TOKEN, signed_claims))
            .domain(self.cookie_domain.clone())
            .expires(
                OffsetDateTime::from_unix_timestamp(exp)
                    .map_err(|source| IdentitySessionError::CookieExpiration { exp, source })?,
            )
            .http_only(true)
            .path("/")
            .same_site(SameSite::Strict)
            .secure(self.cookie_secure)
            .build()
            .into_owned())
    }

    pub fn access_token_removal_cookie(&self) -> Result<Cookie<'static>, IdentitySessionError> {
        let mut cookie = Cookie::build((COOKIE_NAME_ACCESS_TOKEN, ""))
            .domain(self.cookie_domain.clone())
            .http_only(true)
            .path("/")
            .same_site(SameSite::Strict)
            .secure(self.cookie_secure)
            .build()
            .into_owned();

        cookie.make_removal();

        Ok(cookie)
    }

    pub fn refresh_token_cookie(
        &self,
        RefreshTokenClaimsSigned { exp, signed_claims }: RefreshTokenClaimsSigned,
    ) -> Result<Cookie<'static>, IdentitySessionError> {
        Ok(Cookie::build((COOKIE_NAME_REFRESH_TOKEN, signed_claims))
            .domain(self.cookie_domain.clone())
            .expires(
                OffsetDateTime::from_unix_timestamp(exp)
                    .map_err(|source| IdentitySessionError::CookieExpiration { exp, source })?,
            )
            .http_only(true)
            .path("/")
            .same_site(SameSite::Strict)
            .secure(self.cookie_secure)
            .build()
            .into_owned())
    }

    pub fn refresh_token_removal_cookie(&self) -> Result<Cookie<'static>, IdentitySessionError> {
        let mut cookie = Cookie::build((COOKIE_NAME_REFRESH_TOKEN, ""))
            .domain(self.cookie_domain.clone())
            .http_only(true)
            .path("/")
            .same_site(SameSite::Strict)
            .secure(self.cookie_secure)
            .build()
            .into_owned();

        cookie.make_removal();

        Ok(cookie)
    }
}

#[cfg(test)]
mod tests {
    use cookie::Cookie;
    use cookie::SameSite;
    use cookie::time::Duration;

    use super::SessionCookieManager;
    use crate::access_token_claims_signed::AccessTokenClaimsSigned;
    use crate::cookie_name_access_token::COOKIE_NAME_ACCESS_TOKEN;
    use crate::cookie_name_refresh_token::COOKIE_NAME_REFRESH_TOKEN;
    use crate::identity_session_error::IdentitySessionError;
    use crate::refresh_token_claims_signed::RefreshTokenClaimsSigned;

    fn assert_hardened(cookie: &Cookie<'static>) {
        assert_eq!(cookie.domain(), Some("example.test"));
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Strict));
        assert_eq!(cookie.secure(), Some(true));
    }

    fn manager() -> SessionCookieManager {
        SessionCookieManager {
            cookie_domain: "example.test".to_string(),
            cookie_secure: true,
        }
    }

    #[test]
    fn builds_an_access_token_cookie() {
        let cookie = manager()
            .access_token_cookie(AccessTokenClaimsSigned {
                exp: 1_700_000_000,
                signed_claims: "signed-access".to_string(),
            })
            .expect("a well-formed access token cookie is built");

        assert_eq!(cookie.name(), COOKIE_NAME_ACCESS_TOKEN);
        assert_eq!(cookie.value(), "signed-access");
        assert!(cookie.expires().is_some());
        assert_hardened(&cookie);
    }

    #[test]
    fn builds_a_refresh_token_cookie() {
        let cookie = manager()
            .refresh_token_cookie(RefreshTokenClaimsSigned {
                exp: 1_700_000_000,
                signed_claims: "signed-refresh".to_string(),
            })
            .expect("a well-formed refresh token cookie is built");

        assert_eq!(cookie.name(), COOKIE_NAME_REFRESH_TOKEN);
        assert_eq!(cookie.value(), "signed-refresh");
        assert!(cookie.expires().is_some());
        assert_hardened(&cookie);
    }

    #[test]
    fn builds_an_access_token_removal_cookie() {
        let cookie = manager()
            .access_token_removal_cookie()
            .expect("a well-formed access token removal cookie is built");

        assert_eq!(cookie.name(), COOKIE_NAME_ACCESS_TOKEN);
        assert_eq!(cookie.value(), "");
        assert_eq!(cookie.max_age(), Some(Duration::ZERO));
        assert_hardened(&cookie);
    }

    #[test]
    fn builds_a_refresh_token_removal_cookie() {
        let cookie = manager()
            .refresh_token_removal_cookie()
            .expect("a well-formed refresh token removal cookie is built");

        assert_eq!(cookie.name(), COOKIE_NAME_REFRESH_TOKEN);
        assert_eq!(cookie.value(), "");
        assert_eq!(cookie.max_age(), Some(Duration::ZERO));
        assert_hardened(&cookie);
    }

    #[test]
    fn rejects_an_out_of_range_access_token_expiration() {
        let error = manager()
            .access_token_cookie(AccessTokenClaimsSigned {
                exp: i64::MAX,
                signed_claims: "signed-access".to_string(),
            })
            .expect_err("an out-of-range expiration is rejected");

        assert!(matches!(
            error,
            IdentitySessionError::CookieExpiration { exp, .. } if exp == i64::MAX
        ));
        assert!(error.to_string().contains("out of range"));
    }

    #[test]
    fn rejects_an_out_of_range_refresh_token_expiration() {
        let error = manager()
            .refresh_token_cookie(RefreshTokenClaimsSigned {
                exp: i64::MAX,
                signed_claims: "signed-refresh".to_string(),
            })
            .expect_err("an out-of-range expiration is rejected");

        assert!(matches!(
            error,
            IdentitySessionError::CookieExpiration { exp, .. } if exp == i64::MAX
        ));
    }
}
